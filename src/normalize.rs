use crate::{Error, raw};
use libc::free;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

pub fn normalize(query: &str) -> crate::Result<String> {
    let input = CString::new(query)?;
    // SAFETY: No documented invariants, so we assume it's safe as long as it
    // gets a valid nul terminated string
    let result = unsafe { raw::pg_query_normalize(input.as_ptr()) };

    match NonNull::new(result.error) {
        Some(err) => {
            // SAFETY: This is either a malloc'd pointer or NULL
            unsafe { free(result.normalized_query.cast()) };
            Err(Error::from_pg_query_error(err))
        }
        None => {
            // SAFETY: This is always valid if no error was returned
            let c_str = unsafe { CStr::from_ptr(result.normalized_query) };
            let string = c_str.to_string_lossy().into_owned();
            // SAFETY: We aren't holding onto any pointers from this now
            unsafe { raw::pg_query_free_normalize_result(result) };
            Ok(string)
        }
    }
}

#[test]
fn test_normalize_does_the_thing() {
    let normalized = normalize("SELECT * FROM users WHERE id = 1").unwrap();
    assert_eq!(normalized, "SELECT * FROM users WHERE id = $1");

    let normalized = normalize("SELECT * FROM users WHERE id = 1 AND name = $1").unwrap();
    assert_eq!(
        normalized,
        "SELECT * FROM users WHERE id = $2 AND name = $1"
    );
}
