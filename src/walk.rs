use crate::pg_error::PgError;
use crate::{Node, raw};
use std::ffi::c_void;
use std::ops::ControlFlow;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// Walks an AST tree, calling `f` for every node in the tree, including `node`
pub fn walk<'a>(node: Node<'a>, mut f: impl FnMut(Node<'a>)) -> crate::Result {
    walk_manual::<()>(node, |n| {
        f(n);
        ControlFlow::Continue(Recurse::Yes)
    })?;
    Ok(())
}

/// Walks an AST tree, allowing the caller to manage the control flow.
/// If ControlFlow::Continue is returned, the value will determine whether to
/// recurse into the children of the current node or not.
/// Upon receiving ControlFlow::Break, iteration will cease, and the callback
/// will not be called again.
///
/// Returns the value of ControlFlow::Break, or None if the entire tree was
/// walked.
pub fn walk_manual<'a, B>(
    node: Node<'a>,
    mut finder: impl FnMut(Node<'a>) -> ControlFlow<B, Recurse>,
) -> crate::Result<Option<B>> {
    let mut result = None;
    walk_expression_tree(node, |node| {
        let res = finder(node);
        res.map_break(|b| result = Some(b))
    })?;
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recurse {
    Yes,
    No,
}

impl Recurse {
    pub fn recurse_if<T>(b: bool) -> ControlFlow<T, Self> {
        ControlFlow::Continue(if b { Self::Yes } else { Self::No })
    }

    pub fn recurse_unless<T>(b: bool) -> ControlFlow<T, Self> {
        Self::recurse_if(!b)
    }
}

// FIXME(sage): Would the optimizer be able to do more if we generate our own
// AST walk in Rust?
fn walk_expression_tree<'a, F>(node: Node<'a>, mut cb: F) -> crate::Result
where
    F: FnMut(Node<'a>) -> ControlFlow<(), Recurse>,
{
    let mut unwind_payload = None;
    // Start by passing the node we're walking to the caller so they don't
    // need to duplicate logic that they might want to apply to both a node
    // and children of the same type.
    if cb(node) != ControlFlow::Continue(Recurse::Yes) {
        return Ok(());
    };
    walk_expression_tree_inner::<'a>(node, |node| {
        match catch_unwind(AssertUnwindSafe(|| cb(node))) {
            Ok(result) => result,
            Err(e) => {
                unwind_payload = Some(e);
                ControlFlow::Break(())
            }
        }
    })?;

    if let Some(payload) = unwind_payload {
        resume_unwind(payload);
    } else {
        Ok(())
    }
}

fn walk_expression_tree_inner<'a, F>(node: Node<'a>, cb: F) -> crate::Result
where
    F: FnMut(Node<'a>) -> ControlFlow<(), Recurse>,
{
    let mut fn_and_error = (cb, raw::Error::null());
    // SAFETY: Nothing holds a pointer to cb after this function returns.
    // PG exceptions are caught and never jump over Rust frames.
    unsafe {
        raw::wrapped_raw_expression_tree_walker_impl(
            node.as_ptr(),
            Some(walk_node_cb::<'a, F>),
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

extern "C" fn walk_node_cb<'a, F>(node: *mut raw::Node, context: *mut c_void) -> bool
where
    F: FnMut(Node<'a>) -> ControlFlow<(), Recurse>,
{
    // SAFETY: This function is only ever called with a pointer allocated in
    // walk_expression_tree
    let (cb, err) = unsafe { &mut *(context as *mut (F, raw::Error)) };
    // SAFETY: PG always calls this with a valid pointer
    let node = unsafe { Node::from_ptr(node) };
    match node {
        Node::None => false,
        // PG can walk lists, but if a struct field is a list it just blindly
        // passes it to the callback. Recurse back into PG rather than making
        // the caller care
        node @ Node::Invalid(n) if n.type_ == raw::NodeTag_T_List => {
            // SAFETY: Caller is responsible for making this safe
            unsafe {
                raw::wrapped_raw_expression_tree_walker_impl(
                    node.as_ptr(),
                    Some(walk_node_cb::<'a, F>),
                    context,
                    &raw mut *err,
                )
            }
        }
        node => match cb(node) {
            ControlFlow::Break(()) => true,
            ControlFlow::Continue(Recurse::Yes) => {
                // SAFETY: Caller is responsible for making this safe
                unsafe {
                    raw::wrapped_raw_expression_tree_walker_impl(
                        node.as_ptr(),
                        Some(walk_node_cb::<'a, F>),
                        context,
                        &raw mut *err,
                    )
                }
            }
            ControlFlow::Continue(Recurse::No) => false,
        },
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
        .unwrap();
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
    })
    .unwrap();
}

#[test]
fn walking_unsupported_node_type_does_not_abort() {
    unsafe { raw::pg_query_init() };
    let raw_node = raw::Node { type_: u32::MAX };
    let node = Node::Invalid(&raw_node);
    let res = walk(node, |_| ());
    assert!(
        res.unwrap_err()
            .to_string()
            .contains("unrecognized node type"),
    );
}

#[test]
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
    let node = unsafe { Node::from_ptr(&raw mut list as *mut raw::Node) };
    let res = walk(node, |_| ());
    assert!(
        res.unwrap_err()
            .to_string()
            .contains("unrecognized node type"),
    );
}

#[test]
fn walk_manual_allows_caller_to_determine_whether_to_recurse() {
    unsafe { raw::pg_query_init() };
    let query = "SELECT (SELECT 1), (SELECT (SELECT 2) FROM (VALUES (1))), (SELECT 3)";
    let tree = crate::parse(query).unwrap();
    let stmt = tree.stmts().next().unwrap();

    let mut select_count = 0;
    // Count select statements, but never recurse if there's a from clause
    walk_manual::<()>(stmt, |node| match node {
        Node::SelectStmt(s) => {
            select_count += 1;
            Recurse::recurse_unless(s.fromClause().len() > 0)
        }
        _ => ControlFlow::Continue(Recurse::Yes),
    })
    .unwrap();
    assert_eq!(4, select_count);
}

#[test]
fn walk_manual_can_return_value_and_halt_iteration() {
    unsafe { raw::pg_query_init() };
    let query = "SELECT 1, 2";
    let tree = crate::parse(query).unwrap();
    let stmt = tree.stmts().next().unwrap();

    let res: Option<i32> = walk_manual(stmt, |node| match node {
        Node::A_Const(c) => match c.val().and_then(|n| n.numeric_value()) {
            Some(2) => panic!("Iteration should have aborted"),
            Some(v) => ControlFlow::Break(v),
            None => unreachable!(),
        },
        _ => ControlFlow::Continue(Recurse::Yes),
    })
    .unwrap();
    assert_eq!(Some(1), res);
}
