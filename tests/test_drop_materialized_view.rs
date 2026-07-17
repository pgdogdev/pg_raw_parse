mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP MATERIALIZED VIEW
// Description: remove a materialized view
// Syntax:
// DROP MATERIALIZED VIEW [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropmaterializedview.html

#[test]
fn drop_materialized_view_parses() {
    run_cases(&[
        r#"DROP MATERIALIZED VIEW mv"#,
        r#"DROP MATERIALIZED VIEW IF EXISTS mv, mv_old CASCADE"#,
        r#"DROP MATERIALIZED VIEW IF EXISTS mv RESTRICT"#,
    ]);
}
