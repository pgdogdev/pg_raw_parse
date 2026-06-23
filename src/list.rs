use crate::{Node, raw};
use std::ffi::c_int;
use std::ptr::NonNull;

#[repr(u32)]
#[derive(Debug)]
pub enum PgList {
    Node {
        length: c_int,
        _max: c_int,
        elements: NonNull<*mut raw::Node>,
    } = raw::NodeTag_T_List,
    Int {
        length: c_int,
        _max: c_int,
        elements: NonNull<c_int>,
    } = raw::NodeTag_T_IntList,
    Oid {
        length: c_int,
        _max: c_int,
        elements: NonNull<raw::Oid>,
    } = raw::NodeTag_T_OidList,
    Xid {
        length: c_int,
        _max: c_int,
        elements: NonNull<raw::TransactionId>,
    } = raw::NodeTag_T_XidList,
}

impl PgList {
    const VALID_TAGS: &[raw::NodeTag] = &[
        raw::NodeTag_T_List,
        raw::NodeTag_T_IntList,
        raw::NodeTag_T_OidList,
        raw::NodeTag_T_XidList,
    ];

    /// Creates a new PgList from a pointer to a List. Panics if type_ isn't
    /// a valid discriminant. Returns None if the pointer is NULL
    ///
    /// SAFETY: The provided lifetime must not outlive the MemoryContext the
    /// List was allocated in
    pub(crate) unsafe fn from_ptr<'a>(ptr: *mut raw::List) -> Option<&'a Self> {
        let ptr = NonNull::new(ptr);
        ptr.map(|ptr| {
            // SAFETY: We've excluded null pointers
            let tag = unsafe { (*ptr.as_ptr()).type_ };
            assert!(Self::VALID_TAGS.contains(&tag));
            // SAFETY: We are asserting the validitiy of the tag.
            unsafe { Self::from_ptr_unchecked(ptr) }
        })
    }

    /// Creates a new PgList from a pointer to a List.
    ///
    /// SAFETY: The caller must ensure that type_ is a valid discriminant
    pub(crate) unsafe fn from_ptr_unchecked<'a>(ptr: NonNull<raw::List>) -> &'a Self {
        // SAFETY: PgList has a memory identical layout to List
        unsafe { ptr.cast().as_ref() }
    }

    pub fn as_node_list(&self) -> Option<impl Iterator<Item = Node<'_>> + ExactSizeIterator> {
        match self {
            Self::Node {
                length, elements, ..
            } => {
                Some(
                    // SAFETY: PgList can never be passed by value, the borrow
                    // checker will ensure the elements pointer hasn't been freed.
                    // PG guarantees that any list has a length >= 1
                    unsafe { std::slice::from_raw_parts(elements.as_ptr(), *length as _) }
                        .iter()
                        // SAFETY: The lifetime given is the lifetime of self
                        .map(|&p| unsafe { Node::from_ptr(p) }),
                )
            }
            _ => None,
        }
    }

    pub fn expect_node_list(&self) -> impl Iterator<Item = Node<'_>> + ExactSizeIterator {
        self.as_node_list()
            .unwrap_or_else(|| panic!("Expected a node list, found {:?}", self))
    }
}

#[cfg(feature = "field_offset_assertions")]
const _: () = {
    use std::mem;
    ["Offset of Node.length"]
        [mem::offset_of!(PgList, Node.length) - mem::offset_of!(raw::List, length)];
    ["Offset of Node._max"]
        [mem::offset_of!(PgList, Node._max) - mem::offset_of!(raw::List, max_length)];
    ["Offset of Node.elements"]
        [mem::offset_of!(PgList, Node.elements) - mem::offset_of!(raw::List, elements)];
    ["Offset of Int.length"]
        [mem::offset_of!(PgList, Int.length) - mem::offset_of!(raw::List, length)];
    ["Offset of Int._max"]
        [mem::offset_of!(PgList, Int._max) - mem::offset_of!(raw::List, max_length)];
    ["Offset of Int.elements"]
        [mem::offset_of!(PgList, Int.elements) - mem::offset_of!(raw::List, elements)];
    ["Offset of Oid.length"]
        [mem::offset_of!(PgList, Oid.length) - mem::offset_of!(raw::List, length)];
    ["Offset of Oid._max"]
        [mem::offset_of!(PgList, Oid._max) - mem::offset_of!(raw::List, max_length)];
    ["Offset of Oid.elements"]
        [mem::offset_of!(PgList, Oid.elements) - mem::offset_of!(raw::List, elements)];
    ["Offset of Xid.length"]
        [mem::offset_of!(PgList, Xid.length) - mem::offset_of!(raw::List, length)];
    ["Offset of Xid._max"]
        [mem::offset_of!(PgList, Xid._max) - mem::offset_of!(raw::List, max_length)];
    ["Offset of Xid.elements"]
        [mem::offset_of!(PgList, Xid.elements) - mem::offset_of!(raw::List, elements)];
};
