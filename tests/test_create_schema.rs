mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_schema_parses() {
    run_cases(&[
        r#"CREATE SCHEMA my_schema"#,
        r#"CREATE SCHEMA IF NOT EXISTS AUTHORIZATION app_user"#,
        r#"CREATE SCHEMA AUTHORIZATION app_user CREATE TABLE t (id integer) CREATE VIEW v AS SELECT 1 AS id"#,
        r#"CREATE SCHEMA my_schema AUTHORIZATION app_user"#,
    ]);
}
