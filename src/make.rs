use crate::AsNodePtr;
#[cfg(test)]
use crate::FromNodePtr;
use crate::mem::MemoryContext;
use crate::raw::{self, *};
use generativity::Id;
use std::any::type_name;
use std::marker::PhantomData;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/make_funcs_raw.rs"));

/// A token used to make nodes allocated onto a specific memory context.
/// This type ensures that nodes from one memory context cannot be used as
/// fields of nodes from another.
///
/// FIXME(sage): These tests don't assert that the failures are lifetime
/// related
///
/// ```compile_fail
/// use pg_raw_parse::make::memory_token;
///
/// memory_token!(c"mem1", mem1);
/// memory_token!(c"mem2", mem2);
/// let node = mem2.make_String(Some("hi"));
/// let _list = mem1.make_List(&[node]); // Fails, node is on mem2
/// ```
///
/// ```
/// use pg_raw_parse::make::memory_token;
///
/// memory_token!(c"mem1", mem1);
/// memory_token!(c"mem2", mem2);
/// let node = mem1.make_String(Some("hi"));
/// let _list = mem1.make_List(&[node]); // Is fine, both nodes are on mem1
/// ```
#[derive(Clone, Copy)]
pub struct MemoryToken<'a> {
    #[doc(hidden)]
    pub mem: &'a MemoryContext,
    #[doc(hidden)]
    pub id: Id<'a>,
}

impl<'a> MemoryToken<'a> {
    #[allow(non_snake_case)]
    pub fn make_List<T>(self, elems: &[Unique<'a, T>]) -> Unique<'a, crate::list::NodeList> {
        if elems.is_empty() {
            Unique(ptr::null_mut(), self.id, PhantomData)
        } else {
            let list_to_copy = raw::List {
                type_: raw::NodeTag_T_List,
                length: elems.len() as _,
                max_length: elems.len() as _,
                elements: elems.as_ptr().cast_mut().cast(),
                initial_elements: raw::__IncompleteArrayField::new(),
            };
            // SAFETY: The given closure never panics, we're passing valid pointers
            let ptr = unsafe { self.mem.within(|| raw::list_copy(&raw const list_to_copy)) };
            // SAFETY: The returned pointer is always a palloc'd list pointer
            Unique(ptr.cast(), self.id, PhantomData)
        }
    }

    /// Performs a deep copy of the given node onto this memory context,
    /// returning a unique pointer to it.
    pub fn make_unique<'b, T: AsNodePtr>(self, node: T) -> Unique<'a, T::ConvertLifetime<'a>> {
        let node_ptr = node.as_ptr();
        let mut err = ptr::null_mut();
        let copied = unsafe {
            self.mem
                .within(|| raw::wrapped_copy_object(node_ptr, &mut err))
        };
        if !err.is_null() {
            panic!("Unable to copy node of type {}", type_name::<T>())
        }

        Unique(copied.cast(), self.id, PhantomData)
    }
}

#[macro_export]
macro_rules! memory_token {
    ($mname:literal, $mem:ident) => {
        let $mem = $crate::mem::MemoryContext::new($mname);
        memory_token!($mem);
    };

    ($mem:ident) => {
        $crate::make_guard!(a);
        let $mem = $crate::make::MemoryToken {
            mem: &$mem,
            id: a.into(),
        };
    };
}

// FIXME(sage): Change to pub(crate) when we have a way to write a compile-fail
// test for invariant lifetimes without making this pub
#[doc(hidden)]
pub use memory_token;

/// A uniquely owned pointer to a node. This is effectively `Box<T>`, but
/// constrained to the lifetime of its memory context.
#[repr(C)]
pub struct Unique<'a, T>(*mut raw::Node, Id<'a>, PhantomData<T>);

impl<'a, T> Unique<'a, T> {
    /// Consume this to get the inner raw node pointer, erasing its lifetime.
    /// The returned pointer should either be stored along side the memory
    /// context, or assigned to the field of a node within the same memory
    /// context.
    pub(crate) fn into_ptr(self) -> *mut raw::Node {
        self.0
    }

    #[cfg(test)]
    fn into_inner(self) -> T
    where
        T: FromNodePtr,
    {
        // SAFETY: Always a valid pointer
        unsafe { T::from_ptr(self.into_ptr()) }
    }
}

#[test]
fn make_empty_list() {
    memory_token!(c"mem", mem);
    let list = mem.make_List::<crate::Node<'_>>(&[]);
    assert!(list.into_ptr().is_null());
}

#[test]
fn copy_null_pointer() {
    let none_node = crate::Node::None;
    let empty_list = &crate::list::EMPTY_LIST;

    memory_token!(c"mem", mem);
    let copy_none = mem.make_unique(none_node);
    assert!(copy_none.into_ptr().is_null());
    let copy_list = mem.make_unique(empty_list);
    assert!(copy_list.into_ptr().is_null());
}

#[test]
fn copy_node() {
    use crate::nodes;

    memory_token!(c"mem1", mem1);
    memory_token!(c"mem2", mem2);

    let s = mem1.make_String(Some("hi")).into_inner();
    let copied_string: Unique<'_, &nodes::String> = mem2.make_unique(s);
    assert_eq!(Some("hi"), copied_string.into_inner().sval());
    let copied_node: Unique<'_, crate::Node<'_>> = mem2.make_unique(crate::Node::String(s));
    assert_eq!(Some("hi"), copied_node.into_inner().as_str());
}
