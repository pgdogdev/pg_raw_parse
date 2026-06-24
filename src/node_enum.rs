use crate::{nodes, raw};

include!(concat!(env!("OUT_DIR"), "/node_enum_raw.rs"));

impl<'a> Node<'a> {
    /// Convert self into a Node list. Returns None if this is not a list node.
    pub fn as_list(&self) -> Option<impl Iterator<Item = Node<'a>> + ExactSizeIterator + use<'a>> {
        use std::ptr::NonNull;

        match self {
            Node::Invalid(n) if n.type_ == raw::NodeTag_T_List => {
                let ptr = NonNull::from_ref(*n).cast();
                // SAFETY: We've checked the tag
                unsafe { crate::PgList::from_ptr_unchecked(ptr) }.as_node_list()
            }
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
    let actual = node.as_list().unwrap().collect::<Vec<_>>();
    std::assert_matches!(actual[..], [Node::Integer(nodes::Integer { ival: 1, .. })]);
}
