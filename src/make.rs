#[cfg(test)]
use crate::FromNodePtr;
use crate::mem::MemoryContext;
use crate::raw::{self, *};
use crate::{AsNodePtr, Owned};
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
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     let node = mem1.make_String(Some("hi"));
///     owned(|mem2| mem2.make_List(&[node])); // Fails, node is on mem1
///     mem1.make_String(Some("lol"))
/// });
/// ```
///
/// ```
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     let node = mem1.make_String(Some("hi"));
///     mem1.make_List(&[node]) // Is fine, both nodes are on mem1
/// });
/// ```
#[derive(Clone, Copy)]
pub struct MemoryToken<'a> {
    mem: &'a MemoryContext,
    id: Id<'a>,
}

impl<'a> MemoryToken<'a> {
    #[allow(non_snake_case)]
    pub fn make_List<T>(self, elems: &[Unique<'a, T>]) -> Unique<'a, &'a crate::list::NodeList> {
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

/// Construct an owned node. A new memory context will be created, and passed
/// to the given function to allocate onto it. The entire arena will be owned
/// by the return value of this function. The given closure may only return data
/// owned by the memory context passed as an argument
///
/// ```compile_fail
/// use pg_raw_parse::make::owned;
///
/// let mut node = None;
/// owned(|mem| {
///     node = Some(mem.make_String(Some("smuggled")));
///     mem.make_String(Some("returned"))
/// });
/// ```
///
/// ```compile_fail
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     owned(|mem2| mem1.make_String(Some("wrong mem")));
///     mem1.make_String(Some("right mem"))
/// });
/// ```
pub fn owned<F, T>(f: F) -> Owned<T>
where
    for<'a> F: FnOnce(MemoryToken<'a>) -> Unique<'a, &'a T>,
{
    let mem = MemoryContext::new(c"pg_raw_parse_owned_node");
    let node = {
        generativity::make_guard!(a);
        let token = MemoryToken {
            mem: &mem,
            id: a.into(),
        };
        f(token).into_ptr()
    };
    Owned::new(mem, node)
}

#[test]
fn make_empty_list() {
    let list = owned(|mem| mem.make_List::<crate::Node<'_>>(&[]));
    assert!(list.as_ptr().is_null());
}

#[test]
fn copy_null_pointer() {
    let none_node = crate::Node::None;
    let empty_list = &crate::list::EMPTY_LIST;

    let copy_list = owned(|mem| {
        let copy_none = mem.make_unique(none_node);
        assert!(copy_none.into_ptr().is_null());
        mem.make_unique(empty_list)
    });
    assert!(copy_list.as_ptr().is_null());
}

#[test]
fn copy_node() {
    let s = owned(|mem| mem.make_String(Some("hi")));
    let copied_string = owned(|mem| {
        let copied_node = mem.make_unique(crate::Node::String(&*s));
        assert_eq!(Some("hi"), copied_node.into_inner().as_str());
        mem.make_unique(&*s)
    });
    assert_eq!(Some("hi"), copied_string.sval());
}
