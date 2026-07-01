use crate::raw;
use std::ffi::CStr;

pub(crate) struct MemoryContext(raw::MemoryContext);

impl MemoryContext {
    pub(crate) fn new(name: &'static CStr) -> Self {
        // SAFETY: No documented invariants
        unsafe {
            raw::pg_query_init();
        }

        // SAFETY: "Names must be constant strings", fulfilled by name being
        // `&'static`.
        let ctx = unsafe {
            raw::AllocSetContextCreateInternal(
                raw::get_top_memory_context(),
                name.as_ptr(),
                raw::ALLOCSET_DEFAULT_MINSIZE as raw::Size,
                raw::ALLOCSET_DEFAULT_INITSIZE as raw::Size,
                raw::ALLOCSET_DEFAULT_MAXSIZE as raw::Size,
            )
        };
        Self(ctx)
    }

    pub(crate) fn from_raw(raw: raw::MemoryContext) -> Self {
        Self(raw)
    }

    /// Runs the given closure in the memory context of self. Will revert to
    /// the previous memory context before returning. If the provided fucntion
    /// panics, all future calls to this function will panic.
    ///
    /// SAFETY: Callers must ensure the provided function never panics
    pub(crate) unsafe fn within<T>(&self, f: impl FnOnce() -> T) -> T {
        // SAFETY: Caller is responsible for ensuring we don't panic while in
        // this memory context
        let prev = unsafe { raw::MemoryContextSwitchTo(self.0) };
        let result = f();
        // SAFETY: Re-entering the previous context is always safe
        unsafe { raw::MemoryContextSwitchTo(prev) };
        result
    }
}

impl Drop for MemoryContext {
    fn drop(&mut self) {
        // SAFETY: All operations that invalidate the context are unsafe
        unsafe {
            raw::MemoryContextDelete(self.0);
        }
    }
}
