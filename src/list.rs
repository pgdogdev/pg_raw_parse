use crate::{Node, raw};
use std::ffi::c_int;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{fmt, slice};

pub(crate) const EMPTY_LIST: NodeList = NodeList {
    _type: raw::NodeTag_T_List,
    length: 0,
    _max: 0,
    elements: NonNull::dangling(),
};

#[repr(C)]
pub struct NodeList {
    _type: raw::NodeTag,
    length: c_int,
    _max: c_int,
    elements: NonNull<*mut raw::Node>,
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    ["Size of NodeList"][size_of::<NodeList>() - size_of::<raw::List>()];
    ["Alignment of NodeList"][align_of::<NodeList>() - align_of::<raw::List>()];
    ["Offset of field: NodeList::type_"]
        [offset_of!(NodeList, _type) - offset_of!(raw::List, type_)];
    ["Offset of field: NodeList::length"]
        [offset_of!(NodeList, length) - offset_of!(raw::List, length)];
    ["Offset of field: NodeList::max_length"]
        [offset_of!(NodeList, _max) - offset_of!(raw::List, max_length)];
    ["Offset of field: NodeList::elements"]
        [offset_of!(NodeList, elements) - offset_of!(raw::List, elements)];
};

impl NodeList {
    fn as_slice(&self) -> &[*mut raw::Node] {
        // SAFETY: PG guarantees that any non-null list pointer has a length > 1
        unsafe { slice::from_raw_parts(self.elements.as_ptr(), self.len()) }
    }

    /// Casts this list to a specific node type. The returned iterator will
    /// panic if it encounters an unexpected node
    pub(crate) fn cast<T>(&self) -> &CastNodeList<T> {
        // SAFETY: CastNodeList is #[repr(transparent)] over NodeList
        unsafe { &*(&raw const *self).cast() }
    }

    #[inline]
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.length as usize
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn first(&self) -> Option<<&Self as IntoIterator>::Item> {
        self.into_iter().next()
    }
}

impl<'a> IntoIterator for &'a NodeList {
    type IntoIter = NodeListIter<'a>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: NodeList is not Copy, and has no public fields. The only safe
        // way to get a reference to this struct is through [Node], which has
        // enforced the lifetime requirements
        NodeListIter {
            iter: self.as_slice().iter(),
            _marker: PhantomData,
        }
    }
}

impl fmt::Debug for NodeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct NodeListIter<'a, T = Node<'a>> {
    iter: slice::Iter<'a, *mut raw::Node>,
    _marker: PhantomData<T>,
}

impl<'a, T> NodeListIter<'a, T> {
    /// SAFETY: 'a must outlive the lifetime of the node
    unsafe fn convert_node(ptr: *mut raw::Node) -> T
    where
        T: TryFrom<Node<'a>>,
        T::Error: fmt::Debug,
    {
        use std::any::type_name;
        // SAFETY: 'a will always outlive the memory context of the node
        let node = unsafe { Node::from_ptr(ptr) };
        T::try_from(node).unwrap_or_else(|e| panic!("Expected a {}, got {:?}", type_name::<T>(), e))
    }

    /// Casts the list to a specific node type
    fn cast<U>(self) -> NodeListIter<'a, U> {
        NodeListIter {
            iter: self.iter,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for NodeListIter<'a, T>
where
    T: TryFrom<Node<'a>>,
    T::Error: fmt::Debug,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: 'a always outlives the Node
        self.iter.next().map(|p| unsafe { Self::convert_node(*p) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'a, T> FusedIterator for NodeListIter<'a, T> where Self: Iterator {}

impl<'a, T> DoubleEndedIterator for NodeListIter<'a, T>
where
    T: TryFrom<Node<'a>>,
    T::Error: fmt::Debug,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        // SAFETY: 'a always outlives the Node
        self.iter
            .next_back()
            .map(|p| unsafe { Self::convert_node(*p) })
    }
}

impl<'a, T> ExactSizeIterator for NodeListIter<'a, T>
where
    Self: Iterator,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T> fmt::Debug for NodeListIter<'a, T>
where
    Self: IntoIterator,
    <Self as IntoIterator>::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, T> Clone for NodeListIter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
pub struct CastNodeList<T> {
    list: NodeList,
    _marker: PhantomData<T>,
}

impl<T> CastNodeList<T> {
    #[inline]
    pub fn iter<'a>(&'a self) -> <&'a Self as IntoIterator>::IntoIter
    where
        &'a Self: IntoIterator,
    {
        self.into_iter()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.list.length as usize
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    #[inline]
    pub fn first<'a>(&'a self) -> Option<<&'a Self as IntoIterator>::Item>
    where
        &'a Self: IntoIterator,
    {
        self.into_iter().next()
    }
}

#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    use crate::nodes::String;
    use std::mem::{align_of, size_of}; // For example

    ["Size of NodeList"][size_of::<CastNodeList<&String>>() - size_of::<raw::List>()];
    ["Alignment of NodeList"][align_of::<CastNodeList<&String>>() - align_of::<raw::List>()];
};

impl<'a, T> IntoIterator for &'a CastNodeList<T>
where
    NodeListIter<'a, T>: Iterator,
{
    type IntoIter = NodeListIter<'a, T>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter().cast()
    }
}

impl<'a, T> fmt::Debug for &'a CastNodeList<T>
where
    Self: IntoIterator,
    <Self as IntoIterator>::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}
