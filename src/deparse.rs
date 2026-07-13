use crate::mem::MemoryContext;
use crate::pg_error::PgError;
use crate::{Node, Result, nodes, raw};
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

pub fn deparse<'a, N: Into<Node<'a>>>(node: N) -> Result<DeparseResult> {
    let stmt;
    let stmt_ref = match node.into() {
        Node::RawStmt(stmt) => stmt,
        node => {
            stmt = nodes::RawStmt::new(node);
            &stmt
        }
    };
    DeparseResult::deparse(stmt_ref)
}

pub fn deparse_stmts<'a, N>(nodes: impl IntoIterator<Item = N>) -> Result<String>
where
    N: Into<Node<'a>>,
{
    let mut iter = nodes.into_iter();
    let mut result = String::new();
    if let Some(first) = iter.next() {
        result.push_str(deparse(first)?.as_str());
    }

    for elem in iter {
        result.push(';');
        result.push_str(deparse(elem)?.as_str());
    }

    Ok(result)
}

#[test]
fn test_deparse() {
    fn run_test(query: &str) {
        let result = crate::parse(query).unwrap();
        let stmt = result.first().unwrap();
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
