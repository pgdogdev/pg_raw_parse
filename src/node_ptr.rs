use crate::raw::{self, NodeTag};
use generativity::Id;
use std::ptr::{self, NonNull};

pub trait AsNodeRef {
    type AsRef<'b>: AsNodePtr;
    type List: List;
}

impl<T: AsNodeRef> AsNodeRef for Option<T> {
    type AsRef<'b> = Option<T::AsRef<'b>>;
    type List = T::List;
}

impl<'a, T: AsNodeRef> AsNodeRef for &'a T {
    type AsRef<'b> = T::AsRef<'b>;
    type List = T::List;
}

impl<'a, T: AsNodeRef> AsNodeRef for &'a mut T {
    type AsRef<'b> = T::AsRef<'b>;
    type List = T::List;
}

/// # Safety
///
/// [AsNodePtr::as_ptr] must convert back to Self when passed to
/// [FromNodePtr::from_ptr]
pub unsafe trait AsNodePtr: AsNodeRef {
    fn as_ptr(self) -> *mut raw::Node;
}

// SAFETY: If &T is a node, Option<&T> is also a Node
unsafe impl<T: AsNodePtr> AsNodePtr for Option<T> {
    fn as_ptr(self) -> *mut raw::Node {
        match self {
            Some(node) => node.as_ptr(),
            None => ptr::null_mut(),
        }
    }
}

pub trait FromNodePtr: Sized {
    /// SAFETY: The caller must provide a valid pointer or NULL
    unsafe fn from_raw(ptr: *mut raw::Node) -> Self {
        let ptr = NonNull::new(ptr);
        // SAFETY: Caller is responsible for making this safe
        unsafe {
            let tag = ptr.map(|p| p.as_ref().type_).unwrap_or_default();
            Self::from_ptr(tag, ptr)
        }
    }

    /// SAFETY: The caller must provide a valid node pointer
    unsafe fn from_ptr(tag: NodeTag, ptr: Option<NonNull<raw::Node>>) -> Self;
}

impl<T: FromNodePtr> FromNodePtr for Option<T> {
    unsafe fn from_ptr(tag: NodeTag, ptr: Option<NonNull<raw::Node>>) -> Self {
        ptr.map(|ptr|
            // SAFETY: Caller is responsible for making this safe
            unsafe { T::from_ptr(tag, Some(ptr)) })
    }
}

pub trait FromNodeMut<'mem> {
    type MutRef<'mutref>;

    /// # Safety
    ///
    /// The caller must provide a valid node pointer that was allocated onto the
    /// MemoryContext referenced by 'mem, or NULL.
    unsafe fn from_raw_mut<'mutref>(
        ptr: &'mutref mut *mut raw::Node,
        id: Id<'mem>,
    ) -> Self::MutRef<'mutref> {
        // SAFETY: Caller is responsible for making this safe
        unsafe {
            let tag = ptr.as_ref().map(|n| n.type_);
            Self::from_ptr_mut(tag, ptr, id)
        }
    }

    /// # Safety
    ///
    /// In addition to all the invariants specified by [`from_raw_mut`], the
    /// tag must always be value of `(**ptr).type_`. It must always be present,
    /// unless the pointer is NULL.
    unsafe fn from_ptr_mut<'mutref>(
        tag: Option<raw::NodeTag>,
        ptr: &'mutref mut *mut raw::Node,
        id: Id<'mem>,
    ) -> Self::MutRef<'mutref>;
}

impl<'mem, T: FromNodeMut<'mem>> FromNodeMut<'mem> for Option<T> {
    type MutRef<'mutref> = Option<T::MutRef<'mutref>>;

    unsafe fn from_ptr_mut<'mutref>(
        tag: Option<raw::NodeTag>,
        ptr: &'mutref mut *mut raw::Node,
        id: Id<'mem>,
    ) -> Self::MutRef<'mutref> {
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Caller is responsible for making this safe
            Some(unsafe { T::from_ptr_mut(tag, ptr, id) })
        }
    }
}

pub trait ConstructableNode: Sized {
    const TAG: NodeTag;

    fn check_tag(tag: NodeTag) {
        if tag != Self::TAG {
            panic!(
                "Expected {}, got tag {}",
                std::any::type_name::<Self>(),
                // FIXME: Would be nice if we could convert tag -> type name
                tag
            );
        }
    }
}

pub trait List: 'static {
    type Elem<'a>: AsNodePtr;
    const EMPTY: &Self;

    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> Option<Self::Elem<'_>>;
    fn slice(&mut self) -> &mut [*mut raw::Node];
}
