use crate::raw;
use std::ptr;

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

pub trait FromNodePtr {
    /// SAFETY: The caller must provide a valid Node pointer or NULL
    unsafe fn from_ptr(ptr: *mut raw::Node) -> Self;
}

impl<T: FromNodePtr> FromNodePtr for Option<T> {
    unsafe fn from_ptr(ptr: *mut raw::Node) -> Self {
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Caller is responsible for making this safe
            Some(unsafe { T::from_ptr(ptr) })
        }
    }
}
