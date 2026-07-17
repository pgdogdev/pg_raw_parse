mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_table_as_parses() {
    run_cases(&[
        r#"CREATE TABLE new_table AS SELECT 1 AS id"#,
        r#"CREATE TEMP TABLE IF NOT EXISTS new_table (id, name) USING heap WITH (fillfactor = 80) ON COMMIT DROP TABLESPACE fastspace AS SELECT 1 AS id, 'x' AS name WITH NO DATA"#,
    ]);
}
