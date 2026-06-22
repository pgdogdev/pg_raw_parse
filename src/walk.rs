use crate::pg_error::PgError;
use crate::{Node, raw};
use std::ffi::c_void;
use std::ops::ControlFlow;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// Walks an AST tree, calling `f` for every node in the tree.
pub fn walk<'a>(node: Node<'a>, mut f: impl FnMut(Node<'a>)) -> crate::Result {
    walk_until::<()>(node, |n| {
        f(n);
        ControlFlow::Continue(())
    })?;
    Ok(())
}

/// Walks an AST tree until ControlFlow::Break is received. Stops iteration at
/// that point, and returns the given value, or None if the entire tree was
/// walked.
pub fn walk_until<'a, B>(
    node: Node<'a>,
    mut finder: impl FnMut(Node<'a>) -> ControlFlow<B>,
) -> crate::Result<Option<B>> {
    let mut result = None;
    walk_expression_tree(node, |node| {
        let res = finder(node);
        result = res.break_value();
        result.is_some()
    })?;
    Ok(result)
}

// FIXME(sage): Would the optimizer be able to do more if we generate our own
// AST walk in Rust?
fn walk_expression_tree<'a, F>(node: Node<'a>, mut cb: F) -> crate::Result
where
    F: FnMut(Node<'a>) -> bool,
{
    let mut unwind_payload = None;
    walk_expression_tree_inner::<'a>(node, |node| {
        match catch_unwind(AssertUnwindSafe(|| cb(node))) {
            Ok(result) => result,
            Err(e) => {
                unwind_payload = Some(e);
                true
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
    F: FnMut(Node<'a>) -> bool,
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
    F: FnMut(Node<'a>) -> bool,
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
        node => {
            cb(node) ||
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
    let res = walk(node, |_| panic!("should never be called"));
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
