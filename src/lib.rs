#![cfg_attr(feature = "field_offset_assertions", feature(offset_of_enum))]
use std::{ffi, fmt, ptr};

pub mod const_val;
mod deparse;
pub mod error;
pub mod list;
pub mod make;
mod mem;
pub mod node_enum;
mod node_ptr;
pub mod nodes;
mod owned;
mod pg_error;
pub mod raw;
pub mod walk;

pub use crate::const_val::ConstValue;
pub use crate::deparse::{DeparseResult, deparse};
pub use crate::error::{Error, Result};
pub use crate::node_enum::Node;
pub use crate::owned::Owned;

pub(crate) use node_ptr::{AsNodePtr, FromNodePtr};

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
        None => Ok(ParseResult { tree, _mem: mem }),
    }
}

pub struct ParseResult {
    tree: TreeAndWarnings,
    _mem: mem::MemoryContext,
}

// SAFETY: No reason this couldn't be sent to another thread
unsafe impl Send for ParseResult {}
// SAFETY: As long as we are don't try to enter the memory context after this
// is constructed, we're all good
unsafe impl Sync for ParseResult {}

impl ParseResult {
    /// Returns the statements that were parsed
    pub fn stmts(&self) -> impl Iterator<Item = Node<'_>> {
        self.raw_stmts().into_iter().map(|s| s.stmt())
    }

    /// Returns the raw statements that were parsed
    pub fn raw_stmts(&self) -> &list::CastNodeList<&nodes::RawStmt> {
        // SAFETY: The memory context of the tree is guaranteed to outlive
        // the lifetime of self. We are returning a lifetime shorter than self.
        unsafe { Node::from_ptr(self.tree.tree.cast()) }
            .expect_node_list()
            .cast()
    }
}

impl fmt::Debug for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParseResult")
            .field("stmts", &&*self.stmts().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

struct TreeAndWarnings {
    tree: *mut raw::List,
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
