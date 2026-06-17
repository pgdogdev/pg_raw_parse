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

#[derive(Error)]
pub struct CError(ptr::NonNull<raw::PgQueryError>);

impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: Any non-null pointer we got from PG here is a valid string
        let err_cstr = unsafe { ffi::CStr::from_ptr((*self.0.as_ptr()).message) };
        // FIXME: Use CStr::display once stable
        write!(f, "{}", err_cstr.to_string_lossy())
    }
}

impl fmt::Debug for CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CError")
            .field(&format_args!("\"{}\"", self))
            .finish()
    }
}

impl Drop for CError {
    fn drop(&mut self) {
        // SAFETY: self.0 is always a valid pointer
        unsafe { raw::pg_query_free_error(self.0.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Leaks a bunch of memory but who cares it's a test
    fn cerror(msg: &ffi::CStr) -> CError {
        let query_error = raw::PgQueryError {
            message: msg.to_owned().into_raw(),
            funcname: ptr::null_mut(),
            filename: ptr::null_mut(),
            lineno: 0,
            cursorpos: 0,
            context: ptr::null_mut(),
        };
        let ptr = Box::into_raw(Box::new(query_error));
        CError(ptr::NonNull::new(ptr).unwrap())
    }

    #[test]
    fn test_display_impl_for_cerror() {
        assert_eq!(cerror(c"hello").to_string(), "hello");
        assert_eq!(cerror(c"world").to_string(), "world");
    }

    #[test]
    fn test_debug_impl_for_cerror() {
        assert_eq!(format!("{:?}", cerror(c"hello")), "CError(\"hello\")");
        assert_eq!(format!("{:?}", cerror(c"world")), "CError(\"world\")");
    }

    #[test]
    fn test_pretty_print_debug_for_cerror() {
        assert_eq!(
            format!("{:#?}", cerror(c"hello")),
            "CError(\n    \"hello\",\n)"
        );
        assert_eq!(
            format!("{:#?}", cerror(c"world")),
            "CError(\n    \"world\",\n)"
        );
    }
}
