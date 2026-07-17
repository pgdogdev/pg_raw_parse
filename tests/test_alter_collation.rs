mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER COLLATION
// Description: change the definition of a collation
// Syntax:
// ALTER COLLATION name REFRESH VERSION
//
// ALTER COLLATION name RENAME TO new_name
// ALTER COLLATION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER COLLATION name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-altercollation.html

#[test]
fn alter_collation_parses() {
    run_cases(&[
        r#"ALTER COLLATION my_collation REFRESH VERSION"#,
        r#"ALTER COLLATION my_collation RENAME TO my_new_collation"#,
        r#"ALTER COLLATION my_collation OWNER TO CURRENT_USER"#,
        r#"ALTER COLLATION my_collation SET SCHEMA public"#,
    ]);
}
