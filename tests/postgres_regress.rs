use pg_raw_parse::{deparse_stmts, parse, raw};
use std::ffi::{CStr, CString};
use std::fs;
use std::path::{Path, PathBuf};

const REGRESS_DIR: &str = "libpg_query/test/sql/postgres_regress";

struct SplitResult(raw::PgQuerySplitResult);

impl SplitResult {
    fn new(sql: &str) -> Self {
        let sql = CString::new(sql).expect("regression SQL must not contain NUL bytes");
        // SAFETY: The scanner catches PostgreSQL errors internally and does not retain `sql`.
        let result = unsafe { raw::pg_query_split_with_scanner(sql.as_ptr()) };
        Self(result)
    }

    fn error(&self) -> Option<String> {
        if self.0.error.is_null() {
            return None;
        }

        // SAFETY: A non-null PgQueryError has a valid, NUL-terminated message until
        // pg_query_free_split_result is called from Drop.
        let message = unsafe { CStr::from_ptr((*self.0.error).message) };
        Some(message.to_string_lossy().into_owned())
    }

    fn ranges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.0.n_stmts as usize).map(|index| {
            // SAFETY: libpg_query allocates n_stmts valid pointers in stmts.
            let stmt = unsafe { &**self.0.stmts.add(index) };
            (stmt.stmt_location as usize, stmt.stmt_len as usize)
        })
    }
}

impl Drop for SplitResult {
    fn drop(&mut self) {
        // SAFETY: This result came from pg_query_split_with_scanner and is freed once.
        unsafe { raw::pg_query_free_split_result(std::ptr::read(&self.0)) }
    }
}

fn regression_files(root: &Path) -> Vec<PathBuf> {
    let mut files = fs::read_dir(root)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", root.display()))
        .map(|entry| {
            entry
                .expect("failed to read regression directory entry")
                .path()
        })
        .filter(|path| path.extension().is_some_and(|extension| extension == "sql"))
        .collect::<Vec<_>>();
    files.sort();
    files
}

// numerology.sql deliberately contains tokens that make PostgreSQL's scanner
// error before pg_query_split_with_scanner can return any ranges. Its only
// semicolons inside a statement are in a dollar-quoted DO body, so this small
// quote-aware splitter lets us exercise those malformed statements too.
fn split_numerology(sql: &str) -> Vec<(usize, usize)> {
    let bytes = sql.as_bytes();
    let mut ranges = Vec::new();
    let mut statement_start = 0;
    let mut index = 0;
    let mut dollar_quote: Option<&[u8]> = None;
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut line_comment = false;
    let mut block_comment_depth = 0;

    while index < bytes.len() {
        if let Some(delimiter) = dollar_quote {
            if bytes[index..].starts_with(delimiter) {
                index += delimiter.len();
                dollar_quote = None;
            } else {
                index += 1;
            }
            continue;
        }

        if line_comment {
            line_comment = bytes[index] != b'\n';
            index += 1;
            continue;
        }

        if block_comment_depth > 0 {
            if bytes[index..].starts_with(b"/*") {
                block_comment_depth += 1;
                index += 2;
            } else if bytes[index..].starts_with(b"*/") {
                block_comment_depth -= 1;
                index += 2;
            } else {
                index += 1;
            }
            continue;
        }

        if single_quoted {
            if bytes[index] == b'\\' {
                index = (index + 2).min(bytes.len());
            } else if bytes[index..].starts_with(b"''") {
                index += 2;
            } else {
                single_quoted = bytes[index] != b'\'';
                index += 1;
            }
            continue;
        }

        if double_quoted {
            if bytes[index..].starts_with(b"\"\"") {
                index += 2;
            } else {
                double_quoted = bytes[index] != b'\"';
                index += 1;
            }
            continue;
        }

        if bytes[index..].starts_with(b"--") {
            line_comment = true;
            index += 2;
        } else if bytes[index..].starts_with(b"/*") {
            block_comment_depth = 1;
            index += 2;
        } else if bytes[index] == b'\'' {
            single_quoted = true;
            index += 1;
        } else if bytes[index] == b'\"' {
            double_quoted = true;
            index += 1;
        } else if bytes[index] == b'$' {
            let tag_end = bytes[index + 1..]
                .iter()
                .position(|byte| *byte == b'$')
                .map(|offset| index + offset + 1);
            let Some(tag_end) = tag_end else {
                index += 1;
                continue;
            };
            let tag = &bytes[index + 1..tag_end];
            let valid_tag = tag.is_empty()
                || (tag[0].is_ascii_alphabetic() || tag[0] == b'_')
                    && tag[1..]
                        .iter()
                        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
            if valid_tag {
                let delimiter = &bytes[index..=tag_end];
                dollar_quote = Some(delimiter);
                index += delimiter.len();
            } else {
                index += 1;
            }
        } else if bytes[index] == b';' {
            if !sql[statement_start..index].trim().is_empty() {
                ranges.push((statement_start, index - statement_start));
            }
            statement_start = index + 1;
            index += 1;
        } else {
            index += 1;
        }
    }

    assert!(dollar_quote.is_none(), "unterminated dollar quote");
    assert!(!single_quoted, "unterminated single quote");
    assert!(!double_quoted, "unterminated double quote");
    assert_eq!(block_comment_depth, 0, "unterminated block comment");
    if !sql[statement_start..].trim().is_empty() {
        ranges.push((statement_start, sql.len() - statement_start));
    }
    ranges
}

#[test]
fn postgres_regression_sql_parses_and_round_trips() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(REGRESS_DIR);
    let files = regression_files(&root);
    assert_eq!(files.len(), 233, "unexpected PostgreSQL regression corpus");

    let mut statements = 0;
    let mut parsed = 0;
    let mut rejected = 0;
    let mut non_utf8_files = 0;
    let mut round_trip_exceptions = 0;

    for path in files {
        let bytes = fs::read(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let sql = match std::str::from_utf8(&bytes) {
            Ok(sql) => sql,
            Err(_)
                if path
                    .file_name()
                    .is_some_and(|name| name == "collate.windows.win1252.sql") =>
            {
                // pg_raw_parse accepts UTF-8 Rust strings. This PostgreSQL fixture
                // intentionally uses the WIN1252 database encoding.
                non_utf8_files += 1;
                continue;
            }
            Err(error) => panic!("unexpected non-UTF-8 file {}: {error}", path.display()),
        };

        // strings.sql begins with deliberately malformed Unicode escapes that also
        // make the scanner fail. This is the same exception used by libpg_query's
        // own PostgreSQL regression-corpus harness.
        let sql = if path.file_name().is_some_and(|name| name == "strings.sql") {
            let offset = sql
                .find("-- bytea\n")
                .expect("strings.sql no longer contains the bytea section");
            &sql[offset..]
        } else {
            sql
        };

        let ranges = if path
            .file_name()
            .is_some_and(|name| name == "numerology.sql")
        {
            split_numerology(sql)
        } else {
            let split = SplitResult::new(sql);
            if let Some(error) = split.error() {
                panic!("failed to split {}: {error}", path.display());
            }
            split.ranges().collect()
        };

        for (location, length) in ranges {
            let end = location
                .checked_add(length)
                .expect("statement range overflowed");
            let query = sql.get(location..end).unwrap_or_else(|| {
                panic!(
                    "invalid statement range {location}..{end} in {}",
                    path.display()
                )
            });
            statements += 1;

            let tree = match parse(query) {
                Ok(tree) => tree,
                Err(_) => {
                    // PostgreSQL's regression corpus intentionally contains invalid
                    // statements. The round-trip checks below cover every statement
                    // accepted by the raw parser.
                    rejected += 1;
                    continue;
                }
            };
            parsed += 1;

            let deparsed = deparse_stmts(tree.stmts()).unwrap_or_else(|error| {
                panic!(
                    "failed to deparse {} at byte {location}: {error}\n{query}",
                    path.display()
                )
            });
            if let Err(error) = parse(&deparsed) {
                // libpg_query deliberately patches out PostgreSQL's param_junk
                // lexer error. It consequently accepts `$0_1`, but its deparser
                // cannot produce a round-trip-safe representation of that AST.
                if path
                    .file_name()
                    .is_some_and(|name| name == "numerology.sql")
                    && query.trim() == "PREPARE p1 AS SELECT $0_1"
                {
                    round_trip_exceptions += 1;
                    continue;
                }
                panic!(
                    "failed to reparse {} at byte {location}: {error}\noriginal: {query}\ndeparsed: {deparsed}",
                    path.display()
                );
            }
        }
    }

    // Pin the acceptance baseline so the test fails if the parser starts
    // rejecting statements that it currently accepts. Revisit these counts
    // when updating the vendored PostgreSQL regression corpus.
    assert_eq!(statements, 49_152, "unexpected statement count");
    assert_eq!(parsed, 47_648, "unexpected accepted-statement count");
    assert_eq!(rejected, 1_504, "unexpected rejected-statement count");
    assert_eq!(non_utf8_files, 1, "unexpected non-UTF-8 corpus files");
    assert_eq!(round_trip_exceptions, 1, "unexpected round-trip exceptions");
    eprintln!(
        "PostgreSQL regression corpus: {statements} statements, {parsed} parsed, {rejected} rejected"
    );
}
