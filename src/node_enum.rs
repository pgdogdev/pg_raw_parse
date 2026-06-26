use crate::{list, nodes, raw};

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
    let int = nodes::Integer {
        type_: raw::NodeTag_T_Integer,
        ival: 1,
    };
    let mut ptr_to_int = &raw const int;
    let mut list = raw::List {
        type_: raw::NodeTag_T_List,
        length: 1,
        max_length: 1,
        elements: &raw mut ptr_to_int as *mut raw::ListCell,
        initial_elements: raw::__IncompleteArrayField::new(),
    };
    let node = unsafe { Node::from_ptr(&raw mut list as _) };
    let actual = node.expect_node_list().into_iter().collect::<Vec<_>>();
    std::assert_matches!(actual[..], [Node::Integer(nodes::Integer { ival: 1, .. })]);
}
