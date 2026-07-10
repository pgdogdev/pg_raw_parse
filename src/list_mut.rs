use crate::make::{MemoryToken, Unique};
use crate::{AsNodePtr, AsNodeRef, FromNodeMut, List, raw};
use generativity::Id;
use std::marker::PhantomData;
use std::ops::Deref;
use std::slice;

pub struct NodeListMut<'mem, 'mutref, T> {
    pub(crate) id: Id<'mem>,
    /// We take `&mut Option<&mut>` because mutation functions return a new
    /// pointer that must be used instead of the original. The pointer may
    /// be null, so we need Option. Although the functions return a new
    /// pointer, they may modify the list in-place, so the inner most
    /// reference must be unique.
    mut_ref: &'mutref mut Option<&'mutref mut T>,
}

impl<'mem, 'mutref, T> NodeListMut<'mem, 'mutref, T> {
    pub(crate) fn new(id: Id<'mem>, mut_ref: &'mutref mut Option<&'mutref mut T>) -> Self {
        Self { id, mut_ref }
    }
}

impl<'mem, 'mutref, T: List> NodeListMut<'mem, 'mutref, T> {
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

    #[inline]
    pub fn replace(&mut self, ptr: Unique<'mem, &T>) {
        let new_list = ptr.into_ptr().cast::<T>();
        // SAFETY: PG will always return a valid pointer or NULL
        *self.mut_ref = unsafe { new_list.as_mut() };
    }
}

impl<'mem, 'mutref, T> NodeListMut<'mem, 'mutref, T>
where
    Option<&'mutref T>: AsNodePtr,
    T: List,
    for<'a> T::Elem<'a>: AsNodeRef<List = T>,
{
    pub fn get(&self, idx: usize) -> Option<T::Elem<'_>> {
        self.mut_ref.as_ref().and_then(|l| l.get(idx))
    }

    /// Assigns the given element to the specified index. Equivalent to `[]=`.
    pub fn set(&mut self, idx: usize, elem: Unique<'mem, T::Elem<'_>>) {
        let slice = self
            .mut_ref
            .as_mut()
            .unwrap_or_else(|| panic!("Index {idx} is out of bounds"))
            .slice();
        slice[idx] = elem.into_ptr();
    }

    pub fn push(&mut self, mem: MemoryToken<'mem>, elem: Unique<'mem, T::Elem<'_>>) {
        let new_ptr = mem.lappend(self.take_ptr(), elem);
        self.replace(new_ptr);
    }

    pub fn insert(&mut self, mem: MemoryToken<'mem>, idx: usize, elem: Unique<'mem, T::Elem<'_>>) {
        assert!(idx <= self.len());
        let new_ptr = mem.list_insert_nth(self.take_ptr(), idx, elem);
        self.replace(new_ptr)
    }

    pub fn extend(&mut self, mem: MemoryToken<'mem>, elems: Unique<'mem, &T>) {
        let new_ptr = mem.list_concat(self.take_ptr(), elems);
        self.replace(new_ptr)
    }

    fn take_ptr(&mut self) -> *mut raw::List {
        self.mut_ref.take().map(|p| &*p).as_ptr().cast()
    }

    pub(crate) fn into_assignment(self) -> *mut *mut raw::Node {
        std::ptr::from_mut(self.mut_ref).cast()
    }
}

impl<'mem, 'mutref, T> Deref for NodeListMut<'mem, 'mutref, T>
where
    T: List,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.mut_ref.as_ref().map(|r| &**r).unwrap_or(T::EMPTY)
    }
}

impl<'mem, 'mutref, 'a, T> IntoIterator for &'a NodeListMut<'mem, 'mutref, T>
where
    &'a T: IntoIterator,
    T: List,
{
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.mut_ref
            .as_ref()
            .map(|list| list.into_iter())
            .unwrap_or_else(|| T::EMPTY.into_iter())
    }
}

impl<'mem, 'mutref, T> IntoIterator for NodeListMut<'mem, 'mutref, T>
where
    T: List,
    NodeIterMut<'mem, 'mutref, T::Elem<'mutref>>: Iterator,
{
    type IntoIter = NodeIterMut<'mem, 'mutref, T::Elem<'mutref>>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self
            .mut_ref
            .as_mut()
            .map(|l| l.slice().iter_mut())
            .unwrap_or([].iter_mut());
        NodeIterMut {
            id: self.id,
            iter,
            _marker: PhantomData,
        }
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct NodeIterMut<'mem, 'mutref, T> {
    id: Id<'mem>,
    iter: slice::IterMut<'mutref, *mut raw::Node>,
    _marker: PhantomData<T>,
}

impl<'mem, 'mutref, T> Iterator for NodeIterMut<'mem, 'mutref, T>
where
    T: FromNodeMut<'mem>,
{
    type Item = T::MutRef<'mutref>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ptr| {
            // SAFETY: Pointer is always valid or NULL
            unsafe { T::from_raw_mut(ptr, self.id) }
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

#[test]
fn list_mut_iter() {
    let strings = crate::make::owned(|mem| {
        let mut list = mem.make_list(&[
            mem.make_string(Some("foo")),
            mem.make_string(Some("bar")),
            mem.make_string(Some("baz")),
        ]);
        for mut s in list.as_mut() {
            s.set_sval(s.sval().map(|s| mem.copy_string(&s.to_uppercase())));
        }
        list
    });

    assert_eq!(
        vec!["FOO", "BAR", "BAZ"],
        strings.iter().filter_map(|s| s.sval()).collect::<Vec<_>>()
    );
}

#[test]
fn get_and_set() {
    crate::make::owned(|mem| {
        let mut list = mem.make_list(&[
            mem.make_string(Some("foo")),
            mem.make_string(Some("bar")),
            mem.make_string(Some("baz")),
        ]);

        assert_eq!(Some("foo"), list.get(0).and_then(|s| s.sval()));
        assert_eq!(Some("bar"), list.get(1).and_then(|s| s.sval()));

        list.as_mut().set(1, mem.make_string(Some("wibble")));

        assert_eq!(Some("foo"), list.get(0).and_then(|s| s.sval()));
        assert_eq!(Some("wibble"), list.get(1).and_then(|s| s.sval()));

        list
    });
}
