use crate::transform::{Transform, TransformClosure};
use crate::{DeparseResult, Node, NodeMut, Owned, deparse, make, nodes, parse};

pub fn normalize(query: &nodes::RawStmt) -> Owned<nodes::RawStmt> {
    make::owned(|mem| {
        let mut copy = mem.make_unique(query);
        let mut param_count = 0;
        TransformClosure::new(|mut node| match &mut *node {
            NodeMut::A_Const(_) => {
                param_count += 1;
                node.replace(mem.make_param_ref(param_count).uncast());
                None
            }
            NodeMut::ParamRef(p) => {
                param_count += 1;
                p.set_number(param_count);
                None
            }
            _ => Some(node),
        })
        .transform_raw_stmt(copy.as_mut());
        copy
    })
}

pub fn normalize_str(query: &str) -> crate::Result<DeparseResult> {
    let tree = parse(query)?;
    if let Some(stmt) = tree.first() {
        deparse(&*normalize(stmt))
    } else {
        deparse(Node::None)
    }
}

#[test]
fn test_normalize_does_the_thing() {
    let normalized = normalize_str("SELECT * FROM users WHERE id = 1").unwrap();
    assert_eq!(normalized.as_str(), "SELECT * FROM users WHERE id = $1");

    let normalized = normalize_str("SELECT * FROM users WHERE id = 1 AND name = $1").unwrap();
    assert_eq!(
        normalized.as_str(),
        "SELECT * FROM users WHERE id = $1 AND name = $2"
    );

    assert!(normalize_str("").is_err());
}
