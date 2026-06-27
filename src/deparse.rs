use crate::mem::MemoryContext;
use crate::pg_error::PgError;
use crate::{Result, nodes, raw};
use std::ptr::{self, NonNull};

pub struct DeparseResult {
    _mem: MemoryContext,
    string_info: raw::StringInfo,
}

impl DeparseResult {
    pub fn deparse(stmt: &nodes::RawStmt) -> Result<Self> {
        let mem = MemoryContext::new(c"pg_raw_deparse");
        let mut err = ptr::null_mut();
        // SAFETY: We never panic
        let string_info = unsafe {
            mem.within(|| raw::wrapped_raw_deparse((&raw const *stmt).cast_mut(), &mut err))
        };

        match NonNull::new(err) {
            Some(err) => Err(PgError::new(mem, err).into()),
            None => Ok(Self {
                _mem: mem,
                string_info,
            }),
        }
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: We're always allocating a valid pointer
        let info = unsafe { &*self.string_info };
        // SAFETY: We're always allocating a valid pointer
        let bytes =
            unsafe { std::slice::from_raw_parts(info.data.cast_const().cast(), info.len as usize) };
        std::str::from_utf8(bytes).expect("PG always returns valid UTF-8")
    }
}

pub fn deparse(stmt: &nodes::RawStmt) -> Result<DeparseResult> {
    DeparseResult::deparse(stmt)
}

#[test]
fn test_deparse() {
    fn run_test(query: &str) {
        let result = crate::parse(query).unwrap();
        let stmt = result.raw_stmts().first().unwrap();
        let deparsed = deparse(stmt).unwrap();

        assert_eq!(query, deparsed.as_str());
    }

    run_test("SELECT 1");
    run_test("SELECT id, name, email FROM users WHERE users.id = 1");
    run_test(
        "UPDATE users SET name = 'Sage', email = 'sage@pgdog.dev' WHERE users.id = 1 AND NOT EXISTS (SELECT 1 FROM users WHERE email = 'sage@pgdog.dev')",
    );
    run_test(
        "INSERT INTO users (name, email) VALUES ('Sage', 'sage@pgdog.dev') ON CONFLICT DO NOTHING",
    );
    run_test("DELETE FROM users WHERE id = 1");
    run_test("SET my_config TO 1");
    run_test("TRUNCATE users");
}
