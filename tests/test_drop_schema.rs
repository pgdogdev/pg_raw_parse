mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP SCHEMA
// Description: remove a schema
// Syntax:
// DROP SCHEMA [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropschema.html

#[test]
fn drop_schema_parses() {
    run_cases(&[
        r#"DROP SCHEMA my_schema"#,
        r#"DROP SCHEMA IF EXISTS my_schema, my_schema_old CASCADE"#,
        r#"DROP SCHEMA IF EXISTS my_schema RESTRICT"#,
    ]);
}
