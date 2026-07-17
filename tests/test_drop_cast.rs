mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP CAST
// Description: remove a cast
// Syntax:
// DROP CAST [ IF EXISTS ] (source_type AS target_type) [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropcast.html

#[test]
fn drop_cast_parses() {
    run_cases(&[
        r#"DROP CAST (integer AS bigint)"#,
        r#"DROP CAST IF EXISTS (integer AS bigint) CASCADE"#,
    ]);
}
