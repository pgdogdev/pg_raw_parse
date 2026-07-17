mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP INDEX
// Description: remove an index
// Syntax:
// DROP INDEX [ CONCURRENTLY ] [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropindex.html

#[test]
fn drop_index_parses() {
    run_cases(&[
        r#"DROP INDEX my_index"#,
        r#"DROP INDEX IF EXISTS my_index, my_index_old CASCADE"#,
        r#"DROP INDEX IF EXISTS my_index RESTRICT"#,
    ]);
}
