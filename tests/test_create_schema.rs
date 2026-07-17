mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE SCHEMA
// Description: define a new schema
// Syntax:
// CREATE SCHEMA schema_name [ AUTHORIZATION role_specification ] [ schema_element [ ... ] ]
// CREATE SCHEMA AUTHORIZATION role_specification [ schema_element [ ... ] ]
// CREATE SCHEMA IF NOT EXISTS schema_name [ AUTHORIZATION role_specification ]
// CREATE SCHEMA IF NOT EXISTS AUTHORIZATION role_specification
//
// where role_specification can be:
//
//     user_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
//
// URL: https://www.postgresql.org/docs/18/sql-createschema.html

#[test]
fn create_schema_parses() {
    run_cases(&[
        r#"CREATE SCHEMA my_schema"#,
        r#"CREATE SCHEMA IF NOT EXISTS AUTHORIZATION app_user"#,
        r#"CREATE SCHEMA AUTHORIZATION app_user CREATE TABLE t (id integer) CREATE VIEW v AS SELECT 1 AS id"#,
        r#"CREATE SCHEMA my_schema AUTHORIZATION app_user"#,
    ]);
}
