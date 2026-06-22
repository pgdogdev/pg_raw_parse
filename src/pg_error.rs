use crate::mem::MemoryContext;
use crate::raw;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Debug, Display};
use std::ptr::NonNull;

pub struct PgError {
    _mem: MemoryContext,
    error_data: ErrorData,
}

impl PgError {
    /// Returns None if the raw error is NULL
    pub(crate) fn from_raw(raw: raw::Error) -> Option<Self> {
        NonNull::new(raw.error_data).map(|error_data| {
            debug_assert!(
                !raw.mem.is_null(),
                "We should never receive non-null ErrorData with a null MemoryContext"
            );
            Self {
                _mem: MemoryContext::from_raw(raw.mem),
                error_data: ErrorData(error_data),
            }
        })
    }
}

impl Error for PgError {}

impl Debug for PgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PgError")
            .field("error_data", &self.error_data)
            .finish_non_exhaustive()
    }
}

impl Display for PgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME(sage): Use CStr::display when stable
        write!(
            f,
            "{}",
            self.error_data.message().unwrap_or(c"").to_string_lossy()
        )
    }
}

struct ErrorData(NonNull<raw::ErrorData>);

impl ErrorData {
    fn as_ref(&self) -> &raw::ErrorData {
        // SAFETY: We are never returning an invalid pointer other than NULL
        unsafe { self.0.as_ref() }
    }

    fn message(&self) -> Option<&CStr> {
        let ptr = self.as_ref().message;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Will always be a valid pointer or NULL
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    fn detail(&self) -> Option<&CStr> {
        let ptr = self.as_ref().detail;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Will always be a valid pointer or NULL
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }
}

impl Debug for ErrorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorData")
            .field("message", &self.message())
            .field("detail", &self.detail())
            .finish_non_exhaustive()
    }
}
