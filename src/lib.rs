pub use crate::error::Error;
use std::ffi;
use std::ptr;

pub mod error;
pub mod list;
mod mem;
pub mod node_enum;
#[allow(warnings)]
pub mod raw;

pub fn parse(sql: &str) -> Result<ParseResult, error::Error> {
    let mem = mem::MemoryContext::new(c"pg_raw_parse");
    let cstring = ffi::CString::new(sql).map_err(error::Error::StatementContainedNul)?;
    // SAFETY: we never panic within the provided block
    let c_result = unsafe {
        mem.within(|| {
            raw::pg_query_raw_parse(
                cstring.as_ptr(),
                raw::PgQueryParseMode_PG_QUERY_PARSE_DEFAULT as _,
            )
        })
    };
    // Any warnings that were emitted during parsing went into a malloc'd
    // buffer, so we need to construct this even if we're going to return Err
    // to ensure that buffer is freed.
    let tree = TreeAndWarnings {
        tree: c_result.tree,
        stderr_buffer: ptr::NonNull::new(c_result.stderr_buffer),
    };
    match ptr::NonNull::new(c_result.error) {
        Some(e) => Err(Error::from_pg_query_error(e)),
        None => Ok(ParseResult { tree, mem }),
    }
}

pub struct ParseResult {
    tree: TreeAndWarnings,
    mem: mem::MemoryContext,
}

struct TreeAndWarnings {
    tree: list::PgList,
    stderr_buffer: Option<ptr::NonNull<ffi::c_char>>,
}

impl Drop for TreeAndWarnings {
    fn drop(&mut self) {
        // tree was created with palloc, so is managed by postgres.
        // stderr_buffer was malloc'd and must be freed
        // SAFETY: libpg_query documents that the caller must free this.
        unsafe {
            if let Some(ptr) = self.stderr_buffer.take() {
                libc::free(ptr.as_ptr() as _);
            }
        }
    }
}
