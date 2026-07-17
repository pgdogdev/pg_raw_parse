mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER SCHEMA
// Description: change the definition of a schema
// Syntax:
// ALTER SCHEMA name RENAME TO new_name
// ALTER SCHEMA name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// URL: https://www.postgresql.org/docs/18/sql-alterschema.html

#[test]
fn alter_schema_parses() {
    run_cases(&[
        r#"ALTER SCHEMA old_schema RENAME TO new_schema"#,
        r#"ALTER SCHEMA old_schema OWNER TO CURRENT_USER"#,
    ]);
}
