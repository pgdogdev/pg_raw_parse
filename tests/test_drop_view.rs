mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP VIEW
// Description: remove a view
// Syntax:
// DROP VIEW [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropview.html

#[test]
fn drop_view_parses() {
    run_cases(&[
        r#"DROP VIEW v"#,
        r#"DROP VIEW IF EXISTS v, v_old CASCADE"#,
        r#"DROP VIEW IF EXISTS v RESTRICT"#,
    ]);
}
