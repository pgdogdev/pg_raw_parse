use crate::{FromNodePtr, mem, raw};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

/// An owned version of a node
pub struct Owned<T> {
    /// The memory context the node is allocated onto
    _mem: mem::MemoryContext,
    /// A pointer to a node allocated onto [_mem]
    ptr: *mut raw::Node,
    _marker: PhantomData<T>,
}

// SAFETY: No reason this couldn't be sent to another thread
unsafe impl<T> Send for Owned<T> {}
// SAFETY: We don't provide a way to re-enter the memory context after this is
// constructed.
unsafe impl<T> Sync for Owned<T> {}

impl<T> Owned<T> {
    pub(crate) fn new(mem: mem::MemoryContext, ptr: *mut raw::Node) -> Self {
        Self {
            _mem: mem,
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Owned<T>
where
    for<'a> &'a T: FromNodePtr,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The lifetime cannot outlive self
        unsafe { FromNodePtr::from_raw(self.ptr) }
    }
}

impl<T> fmt::Debug for Owned<T>
where
    Self: Deref,
    for<'a> &'a <Self as Deref>::Target: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self).deref().fmt(f)
    }
}
