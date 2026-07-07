use crate::{AsNodePtr, ConstructableNode, FromNodeMut, FromNodePtr, list, nodes, raw};
use generativity::Id;
use std::ptr::NonNull;

include!(concat!(env!("OUT_DIR"), "/node_enum_raw.rs"));

impl<'a> Node<'a> {
    /// Get the node list value of self. Returns None if self is not a NodeList
    #[inline]
    pub fn as_node_list(&self) -> Option<&'a list::NodeList> {
        match self {
            // Empty lists are represented as null pointers
            Self::None => Some(&list::EMPTY_LIST),
            Self::NodeList(l) => Some(l),
            _ => None,
        }
    }

    /// Get the node list value of self. Panics if self is not a NodeList
    #[inline]
    pub fn expect_node_list(&self) -> &'a list::NodeList {
        self.as_node_list()
            .unwrap_or_else(|| panic!("Expected a node list, found {:?}", self))
    }

    /// Get the string value of self. Returns None if this is not a
    /// Node::String
    #[inline]
    pub fn as_str(self) -> Option<&'a str> {
        match self {
            Node::String(s) => s.sval(),
            _ => None,
        }
    }
}

#[test]
fn test_node_as_list() {
    use crate::make::*;

    let list = owned(|mem| {
        let int = mem.make_integer(1);
        mem.make_list(&[int.uncast()])
    });
    let node = Node::NodeList(&list);
    let actual = node.expect_node_list().into_iter().collect::<Vec<_>>();
    std::assert_matches!(actual[..], [Node::Integer(nodes::Integer { ival: 1, .. })]);
}
