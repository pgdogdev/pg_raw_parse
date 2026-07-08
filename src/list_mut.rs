use crate::make::{MemoryToken, Unique};
use crate::{AsNodePtr, AsNodeRef, FromNodeMut, List, raw};
use generativity::Id;
use std::marker::PhantomData;
use std::slice;

pub struct NodeListMut<'a, 'b, T> {
    pub(crate) id: Id<'a>,
    /// We take `&mut Option<&mut>` because mutation functions return a new
    /// pointer that must be used instead of the original. The pointer may
    /// be null, so we need Option. Although the functions return a new
    /// pointer, they may modify the list in-place, so the inner most
    /// reference must be unique.
    mut_ref: &'b mut Option<&'b mut T>,
}

impl<'a, 'b, T> NodeListMut<'a, 'b, T> {
    pub(crate) fn new(id: Id<'a>, mut_ref: &'b mut Option<&'b mut T>) -> Self {
        Self { id, mut_ref }
    }
}

impl<'a, 'b, T: List> NodeListMut<'a, 'b, T> {
    #[inline]
    pub fn iter<'c>(&'c self) -> <&'c Self as IntoIterator>::IntoIter
    where
        &'c Self: IntoIterator,
    {
        self.into_iter()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.mut_ref
            .as_ref()
            .map(|list| list.len())
            .unwrap_or_default()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, 'b, T> NodeListMut<'a, 'b, T>
where
    for<'c> Option<&'b T>: AsNodePtr,
    T: List,
    T::Elem<'a>: AsNodeRef<List = T>,
{
    pub fn push(&mut self, mem: MemoryToken<'a>, elem: Unique<'a, T::Elem<'a>>) {
        let new_ptr = mem.lappend(self.take_ptr(), elem);
        self.replace_ptr(new_ptr);
    }

    pub fn insert(&mut self, mem: MemoryToken<'a>, idx: usize, elem: Unique<'a, T::Elem<'a>>) {
        assert!(idx <= self.len());
        let new_ptr = mem.list_insert_nth(self.take_ptr(), idx, elem);
        self.replace_ptr(new_ptr)
    }

    pub fn extend(&mut self, mem: MemoryToken<'a>, elems: Unique<'a, &'a T>) {
        let new_ptr = mem.list_concat(self.take_ptr(), elems);
        self.replace_ptr(new_ptr)
    }

    fn replace_ptr(&mut self, ptr: Unique<'a, &'a T>) {
        let new_list = ptr.into_ptr().cast::<T>();
        // SAFETY: PG will always return a valid pointer or NULL
        *self.mut_ref = unsafe { new_list.as_mut() };
    }

    pub(crate) fn take_ptr(&mut self) -> *mut raw::List {
        self.mut_ref.take().map(|p| &*p).as_ptr().cast()
    }
}

impl<'a, 'b, 'c, T> IntoIterator for &'c NodeListMut<'a, 'b, T>
where
    &'c T: IntoIterator,
    T: List,
{
    type IntoIter = <&'c T as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.mut_ref
            .as_ref()
            .map(|list| list.into_iter())
            .unwrap_or_else(|| T::EMPTY.into_iter())
    }
}

impl<'a, 'b, T> IntoIterator for NodeListMut<'a, 'b, T>
where
    T: List,
    NodeIterMut<'a, 'b, T::Elem<'b>>: Iterator,
{
    type IntoIter = NodeIterMut<'a, 'b, T::Elem<'b>>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct NodeIterMut<'a, 'b, T> {
    id: Id<'a>,
    iter: slice::IterMut<'b, *mut raw::Node>,
    _marker: PhantomData<T>,
}

impl<'a, 'b, T> Iterator for NodeIterMut<'a, 'b, T>
where
    T: FromNodeMut<'a>,
{
    type Item = T::MutRef<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ptr| {
            // SAFETY: Pointer is always valid or NULL
            unsafe { T::from_ptr_mut(ptr, self.id) }
        })
    }
}

#[test]
fn list_push() {
    let list = crate::make::owned(|mem| {
        let mut cast_list = mem.cast_empty::<crate::nodes::String>();
        cast_list.as_mut().push(mem, mem.make_string(Some("hi")));
        cast_list
    });

    assert_eq!(1, list.len());
    assert_eq!(Some("hi"), list.first().unwrap().sval());

    let list = crate::make::owned(|mem| {
        let mut uncast_list = mem.empty();
        uncast_list
            .as_mut()
            .push(mem, mem.make_string(Some("hi")).uncast());
        uncast_list
    });

    assert_eq!(1, list.len());
    assert_eq!(Some("hi"), list.first().unwrap().as_str());
}
