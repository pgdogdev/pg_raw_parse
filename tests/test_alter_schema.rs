mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_schema_parses() {
    run_cases(&[
        r#"ALTER SCHEMA old_schema RENAME TO new_schema"#,
        r#"ALTER SCHEMA old_schema OWNER TO CURRENT_USER"#,
    ]);
}
