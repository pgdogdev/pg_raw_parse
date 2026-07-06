use crate::pg_error::PgError;
use crate::{AsNodePtr, FromNodePtr, Node, NodeMut, nodes, raw};
use std::ffi::c_void;
use std::ops::ControlFlow;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::ptr::NonNull;

/// Walks an AST tree, calling `f` for every node in the tree, including `node`
pub fn walk<'a>(node: Node<'a>, mut f: impl FnMut(Node<'a>)) {
    f(node);
    walk_manual::<()>(node, |n| {
        f(n);
        ControlFlow::Continue(Recurse::Yes)
    });
}

/// Calls `callback` for each child of this AST, allowing fine grained control
/// over the control flow. If ControlFlow::Continue is returned, the value will
/// determine whether to recurse into the children of the current node or not.
///
/// Upon receiving ControlFlow::Break, iteration will cease, and the callback
/// will not be called again.
///
/// ## Examples
///
/// ```
/// use pg_raw_parse::{Node, walk::{Recurse, walk_manual}};
///
/// let query = "SELECT (SELECT 1), (SELECT (SELECT 2) FROM (VALUES (1))), (SELECT 3)";
/// let tree = pg_raw_parse::parse(query).unwrap();
/// let stmt = tree.stmts().next().unwrap();
///
/// let mut select_count = 0;
/// // Count select statements, but never recurse if there's a from clause
/// walk_manual::<()>(stmt, |node| match node {
///     Node::SelectStmt(s) => {
///         select_count += 1;
///         Recurse::recurse_unless(s.fromClause().len() > 0)
///     }
///     _ => Recurse::yes(),
/// });
/// assert_eq!(3, select_count);
/// ```
///
/// ```
/// use std::ops::ControlFlow;
/// use pg_raw_parse::{Node, walk::{Recurse, walk_manual}};
///
/// let query = "SELECT 1, 2";
/// let tree = pg_raw_parse::parse(query).unwrap();
/// let stmt = tree.stmts().next().unwrap();
///
/// let res: Option<i32> = walk_manual(stmt, |node| match node {
///     Node::A_Const(c) => match c.val().and_then(|n| n.numeric_value()) {
///         Some(2) => unreachable!(),
///         Some(v) => ControlFlow::Break(v),
///         None => unreachable!(),
///     },
///     _ => Recurse::yes(),
/// });
/// assert_eq!(Some(1), res);
/// ```
pub fn walk_manual<'a, B>(
    node: Node<'a>,
    mut callback: impl FnMut(Node<'a>) -> ControlFlow<B, Recurse>,
) -> Option<B> {
    let mut result = None;
    walk_expression_tree(node, |node| {
        let res = callback(node);
        res.map_break(|b| result = Some(b))
    });
    result
}

/// Walk an AST, calling the callback with a mutable reference for each node.
/// The compiler will prevent the parent node from being modified while this
/// walk is in progress.
///
/// ```compile_fail
/// use pg_raw_parse::{parse, make, nodes, walk, NodeMut};
///
/// make::owned(|mem| {
///     let mut select = mem.make_node::<nodes::SelectStmt>();
///     walk::walk_mut(select.as_mut().into(), |node| {
///         // Accessing the outer select, not the node being walked. Not okay.
///         select.as_mut().set_fromClause(mem.empty());
///     });
///     select
/// });
pub fn walk_mut<'a, 'b, F>(node: NodeMut<'a, 'b>, mut f: F)
where
    for<'c> F: FnMut(NodeMut<'a, 'c>),
{
    walk_expression_tree_mut(node, |node| {
        f(node);
        Recurse::yes()
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recurse {
    Yes,
    No,
}

impl Recurse {
    /// Recurse into children of this node. You should return this by default
    /// unless you're manually recursing into the current node
    #[inline]
    pub fn yes<T>() -> ControlFlow<T, Self> {
        ControlFlow::Continue(Self::Yes)
    }

    /// Continue walking the tree, but do not recurse into the current node's
    /// children. Return this if you're manually recursing or you simply want to
    /// ignore nodes of a certain type
    #[inline]
    pub fn no<T>() -> ControlFlow<T, Self> {
        ControlFlow::Continue(Self::No)
    }

    #[inline]
    pub fn recurse_if<T>(b: bool) -> ControlFlow<T, Self> {
        if b { Self::yes() } else { Self::no() }
    }

    #[inline]
    pub fn recurse_unless<T>(b: bool) -> ControlFlow<T, Self> {
        Self::recurse_if(!b)
    }
}

// FIXME(sage): Would the optimizer be able to do more if we generate our own
// AST walk in Rust?
fn walk_expression_tree<'a, F>(node: Node<'a>, mut cb: F)
where
    F: FnMut(Node<'a>) -> ControlFlow<(), Recurse>,
{
    walk_catching_unwind(node.as_ptr(), |node| {
        // SAFETY: If we had a valid node come in, all its children should
        // also be valid.
        let node = unsafe { Node::from_raw(node.as_ptr()) };
        cb(node)
    })
}

fn walk_expression_tree_mut<'a, 'b, F>(node: NodeMut<'a, 'b>, mut cb: F)
where
    for<'c> F: FnMut(NodeMut<'a, 'c>) -> ControlFlow<(), Recurse>,
{
    let id = node.id();
    walk_catching_unwind(node.into_ptr(), |node| {
        // SAFETY: If we had a valid node come in, all its children should
        // also be valid.
        let node = unsafe { NodeMut::from_raw(node, id) };
        cb(node)
    })
}

fn walk_catching_unwind<F>(node: *mut raw::Node, mut cb: F)
where
    F: FnMut(NonNull<raw::Node>) -> ControlFlow<(), Recurse>,
{
    let mut unwind_payload = None;
    walk_expression_tree_inner(node, |node| {
        match catch_unwind(AssertUnwindSafe(|| cb(node))) {
            Ok(result) => result,
            Err(e) => {
                unwind_payload = Some(e);
                ControlFlow::Break(())
            }
        }
    })
    .expect("failed to walk expression tree");

    if let Some(payload) = unwind_payload {
        resume_unwind(payload);
    }
}

fn walk_expression_tree_inner<F>(mut node: *mut raw::Node, cb: F) -> crate::Result
where
    F: FnMut(NonNull<raw::Node>) -> ControlFlow<(), Recurse>,
{
    let mut fn_and_error = (cb, raw::Error::null());
    // SAFETY: Nothing holds a pointer to cb after this function returns.
    // PG exceptions are caught and never jump over Rust frames.
    unsafe {
        if (*node).type_ == raw::NodeTag_T_RawStmt {
            node = <&nodes::RawStmt>::from_raw(node).stmt().as_ptr()
        }
        raw::wrapped_raw_expression_tree_walker_impl(
            node,
            Some(walk_node_cb::<F>),
            &raw mut fn_and_error as *mut c_void,
            &raw mut fn_and_error.1,
        );
    }

    if let Some(e) = PgError::from_raw(fn_and_error.1) {
        Err(e.into())
    } else {
        Ok(())
    }
}

extern "C" fn walk_node_cb<F>(node: *mut raw::Node, context: *mut c_void) -> bool
where
    F: FnMut(NonNull<raw::Node>) -> ControlFlow<(), Recurse>,
{
    // SAFETY: This function is only ever called with a pointer allocated in
    // walk_expression_tree
    let (cb, err) = unsafe { &mut *(context as *mut (F, raw::Error)) };

    let Some(node) = NonNull::new(node) else {
        return false;
    };

    match cb(node) {
        ControlFlow::Break(()) => true,
        ControlFlow::Continue(Recurse::Yes) => {
            // SAFETY: Caller is responsible for making this safe
            unsafe {
                raw::wrapped_raw_expression_tree_walker_impl(
                    node.as_ptr(),
                    Some(walk_node_cb::<F>),
                    context,
                    &raw mut *err,
                )
            }
        }
        ControlFlow::Continue(Recurse::No) => false,
    }
}

#[test]
fn test_walking_entire_ast() {
    let tree = crate::parse(
        "SELECT * FROM users \
            WHERE id = $1 \
            AND email = $2 \
            AND id IN \
            (SELECT user_id \
            FROM users_who_opted_into_stuff \
            WHERE user_id = $1)",
    )
    .unwrap();
    let mut c = 0;
    let mut m = 0;
    for stmt in tree.stmts() {
        walk(stmt, |n| {
            if let Node::ParamRef(p) = n {
                c += 1;
                m = m.max(p.number);
            }
        })
    }
    assert_eq!(c, 3);
    assert_eq!(m, 2);
}

#[test]
#[should_panic = "look ma, no abort!"]
fn panicking_in_walk_does_not_abort() {
    let parsed = crate::parse("SELECT 1").unwrap();
    walk(parsed.stmts().nth(0).unwrap(), |_| {
        panic!("look ma, no abort!")
    });
}

#[test]
#[should_panic = "unrecognized node type"]
fn walking_unsupported_node_type_does_not_abort() {
    unsafe { raw::pg_query_init() };
    let raw_node = raw::Node { type_: u32::MAX };
    let node = Node::Invalid(&raw_node);
    walk(node, |_| ());
}

#[test]
#[should_panic = "unrecognized node type"]
fn error_is_set_after_recursion() {
    unsafe {
        raw::pg_query_init();
    }
    let mut invalid_node = raw::Node { type_: u32::MAX };
    let mut ptr_to_invalid_node = &raw mut invalid_node;
    let mut list = raw::List {
        type_: raw::NodeTag_T_List,
        length: 1,
        max_length: 1,
        elements: &raw mut ptr_to_invalid_node as *mut raw::ListCell,
        initial_elements: raw::__IncompleteArrayField::new(),
    };
    // SAFETY: It's a stack pointer. It's fine.
    let node = unsafe { Node::from_raw((&raw mut list).cast()) };
    walk(node, |_| ());
}

#[test]
fn walk_mutable_tree() {
    use crate::{deparse, make, parse};

    let tree = parse(
        "WITH lol AS (INSERT INTO users VALUES (bar(2))) SELECT foo(1) FROM users WHERE id = baz(3)",
    )
    .unwrap();
    let stmt = tree.into_iter().next().unwrap();
    let fooified = make::owned(|mem| {
        let mut copy = mem.make_unique(stmt);
        walk_mut(copy.as_mut().into(), |node| match node {
            NodeMut::FuncCall(mut f) => {
                // FIXME(sage): We need a more ergonomic way to mutate lists.
                // At absolute minimum we need a way to get
                // `Vec<Unique<'_, Node>>` from a list field of a
                // `Unique<'a, T>` that doesn't require copying.
                let old_name = f.funcname().into_iter().map(|n| mem.make_unique(n));
                let mut new_name = vec![mem.make_String(Some("foo")).uncast()];
                new_name.extend(old_name);
                f.set_funcname(mem.make_List(&new_name));
            }
            _ => (),
        });
        copy
    });
    assert_eq!(
        deparse(&*fooified).unwrap().as_str(),
        "WITH lol AS (INSERT INTO users VALUES (foo.bar(2))) SELECT foo.foo(1) FROM users WHERE id = foo.baz(3)"
    );
}
