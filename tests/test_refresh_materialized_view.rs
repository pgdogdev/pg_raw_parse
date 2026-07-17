mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     REFRESH MATERIALIZED VIEW
// Description: replace the contents of a materialized view
// Syntax:
// REFRESH MATERIALIZED VIEW [ CONCURRENTLY ] name
//     [ WITH [ NO ] DATA ]
//
// URL: https://www.postgresql.org/docs/18/sql-refreshmaterializedview.html

#[test]
fn refresh_materialized_view_parses() {
    run_cases(&[
        r#"REFRESH MATERIALIZED VIEW mv"#,
        r#"REFRESH MATERIALIZED VIEW CONCURRENTLY mv WITH NO DATA"#,
        r#"REFRESH MATERIALIZED VIEW mv WITH DATA"#,
    ]);
}
