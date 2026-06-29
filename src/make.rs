use crate::mem::MemoryContext;
use crate::raw::{self, *};
use generativity::Id;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/make_funcs_raw.rs"));

/// FIXME(sage): These tests don't assert that the failures are lifetime
/// related
///
/// ```compile_fail
/// use pg_raw_parse::mem::MemoryContext;
/// use pg_raw_parse::make::{memory_token, make_String, make_List};
///
/// let mem1 = MemoryContext::new(c"mem1");
/// let mem2 = MemoryContext::new(c"mem2");
/// memory_token!(mem1);
/// memory_token!(mem2);
/// let node = make_String(mem2, Some("hi"));
/// let _list = make_List(mem1, &[node]); // Fails, node is on mem2
/// ```
///
/// ```
/// use pg_raw_parse::mem::MemoryContext;
/// use pg_raw_parse::make::{memory_token, make_String, make_List};
///
/// let mem1 = MemoryContext::new(c"mem1");
/// let mem2 = MemoryContext::new(c"mem2");
/// memory_token!(mem1);
/// memory_token!(mem2);
/// let node = make_String(mem1, Some("hi"));
/// let _list = make_List(mem1, &[node]); // Is fine, both nodes are on mem1
/// ```
// FIXME(sage): Change to pub(crate) when we have a way to write a compile-fail
// test for invariant lifetimes without making this pub
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct MemoryToken<'a> {
    #[doc(hidden)]
    pub mem: &'a MemoryContext,
    #[doc(hidden)]
    pub id: Id<'a>,
}

#[macro_export]
macro_rules! memory_token {
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
pub struct Unique<'a, T>(Option<&'a mut T>, Id<'a>);

impl<'a, T> Unique<'a, T> {
    /// Consume this to get the inner raw node pointer, erasing its lifetime.
    /// The returned pointer should either be stored along side the memory
    /// context, or assigned to the field of a node within the same memory
    /// context.
    pub(crate) fn into_ptr(self) -> *mut raw::Node {
        self.0.map(ptr::from_mut).unwrap_or(ptr::null_mut()).cast()
    }
}

// FIXME(sage): Change to pub(crate) when we have a way to write a compile-fail
// test for invariant lifetimes without making this pub
#[doc(hidden)]
#[allow(non_snake_case)]
pub fn make_List<'a, T>(
    mem: MemoryToken<'a>,
    elems: &[Unique<'a, T>],
) -> Unique<'a, crate::list::NodeList> {
    if elems.is_empty() {
        Unique(None, mem.id)
    } else {
        let list_to_copy = raw::List {
            type_: raw::NodeTag_T_List,
            length: elems.len() as _,
            max_length: elems.len() as _,
            elements: elems.as_ptr().cast_mut().cast(),
            initial_elements: raw::__IncompleteArrayField::new(),
        };
        // SAFETY: The given closure never panics, we're passing valid pointers
        let ptr = unsafe { mem.mem.within(|| raw::list_copy(&raw const list_to_copy)) };
        // SAFETY: The returned pointer is always a palloc'd list pointer
        Unique(Some(unsafe { &mut *ptr.cast() }), mem.id)
    }
}
