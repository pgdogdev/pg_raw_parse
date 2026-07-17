mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_schema_parses() {
    run_cases(&[
        r#"DROP SCHEMA my_schema"#,
        r#"DROP SCHEMA IF EXISTS my_schema, my_schema_old CASCADE"#,
        r#"DROP SCHEMA IF EXISTS my_schema RESTRICT"#,
    ]);
}
