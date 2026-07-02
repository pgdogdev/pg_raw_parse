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

pub(crate) use node_ptr::{AsNodePtr, ConstructableNode, FromNodeMut, FromNodePtr};

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
    let warnings = Warnings {
        stderr_buffer: ptr::NonNull::new(c_result.stderr_buffer),
    };
    match ptr::NonNull::new(c_result.error) {
        Some(e) => Err(Error::from_pg_query_error(e)),
        None => Ok(ParseResult {
            _warnings: warnings,
            tree: Owned::new(mem, c_result.tree.cast()),
        }),
    }
}

pub type StmtList = list::CastNodeList<nodes::RawStmt>;

pub struct ParseResult {
    _warnings: Warnings,
    tree: Owned<StmtList>,
}

impl ParseResult {
    /// Returns the statements that were parsed
    pub fn stmts(&self) -> impl Iterator<Item = Node<'_>> {
        self.raw_stmts().into_iter().map(|s| s.stmt())
    }

    /// Returns the raw statements that were parsed
    pub fn raw_stmts(&self) -> &StmtList {
        &self.tree
    }

    /// Returns the list of raw statements that were parsed, discarding any
    /// warnings
    pub fn into_inner(self) -> Owned<StmtList> {
        self.tree
    }
}

impl fmt::Debug for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParseResult")
            .field("stmts", &&*self.stmts().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

struct Warnings {
    stderr_buffer: Option<ptr::NonNull<ffi::c_char>>,
}

impl Drop for Warnings {
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
