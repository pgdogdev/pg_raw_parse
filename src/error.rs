use crate::raw;
use std::{ffi, fmt, ptr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// An error was returned from pg_parse
    #[error("Invalid statement: {0}")]
    Parse(#[from] CError),
    /// Statement contianed nul bytes
    #[error("Statement contained nul bytes: {0}")]
    StatementContainedNul(#[from] ffi::NulError),
}

impl Error {
    pub(crate) fn from_pg_query_error(ptr: ptr::NonNull<raw::PgQueryError>) -> Self {
        Self::Parse(CError(ptr))
    }
}

// FIXME(sage): Lol this is the most useless debug derive of all time
#[derive(Debug, Error)]
pub struct CError(ptr::NonNull<raw::PgQueryError>);

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: Any non-null pointer we got from PG here is a valid string
        let err_cstr = unsafe { ffi::CStr::from_ptr((*self.0.as_ptr()).message) };
        // FIXME: Use CStr::display once stable
        write!(
            f,
            "{}",
            err_cstr.to_str().expect("error message valid UTF-8")
        )
    }
}

impl Drop for CError {
    fn drop(&mut self) {
        // SAFETY: self.0 is always a valid pointer
        unsafe { raw::pg_query_free_error(self.0.as_ptr()) }
    }
}
