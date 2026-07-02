use crate::raw::{self, NodeTag};
use std::ptr::{self, NonNull};

/// [AsNodePtr::as_ptr] must convert back to Self when passed to
/// [FromNodePtr::from_ptr]
pub unsafe trait AsNodePtr {
    type ConvertLifetime<'b>;

    fn as_ptr(self) -> *mut raw::Node;
}

// SAFETY: If &T is a node, Option<&T> is also a Node
unsafe impl<T: AsNodePtr> AsNodePtr for Option<T> {
    type ConvertLifetime<'b> = Option<T::ConvertLifetime<'b>>;

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
