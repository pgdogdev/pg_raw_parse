mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_materialized_view_parses() {
    run_cases(&[
        r#"CREATE MATERIALIZED VIEW mv AS SELECT 1 AS col"#,
        r#"CREATE MATERIALIZED VIEW IF NOT EXISTS mv (col) USING heap WITH (fillfactor = 80) TABLESPACE fastspace AS SELECT 1 WITH NO DATA"#,
    ]);
}
