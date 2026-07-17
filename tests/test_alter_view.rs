mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER VIEW
// Description: change the definition of a view
// Syntax:
// ALTER VIEW [ IF EXISTS ] name ALTER [ COLUMN ] column_name SET DEFAULT expression
// ALTER VIEW [ IF EXISTS ] name ALTER [ COLUMN ] column_name DROP DEFAULT
// ALTER VIEW [ IF EXISTS ] name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER VIEW [ IF EXISTS ] name RENAME [ COLUMN ] column_name TO new_column_name
// ALTER VIEW [ IF EXISTS ] name RENAME TO new_name
// ALTER VIEW [ IF EXISTS ] name SET SCHEMA new_schema
// ALTER VIEW [ IF EXISTS ] name SET ( view_option_name [= view_option_value] [, ... ] )
// ALTER VIEW [ IF EXISTS ] name RESET ( view_option_name [, ... ] )
//
// URL: https://www.postgresql.org/docs/18/sql-alterview.html

#[test]
fn alter_view_parses() {
    run_cases(&[
        r#"ALTER VIEW IF EXISTS v ALTER COLUMN col SET DEFAULT 1"#,
        r#"ALTER VIEW v ALTER COLUMN col DROP DEFAULT"#,
        r#"ALTER VIEW v OWNER TO CURRENT_USER"#,
        r#"ALTER VIEW v RENAME TO v_new"#,
        r#"ALTER VIEW v SET SCHEMA public"#,
        r#"ALTER VIEW v SET (security_barrier = true, check_option = local)"#,
        r#"ALTER VIEW v RESET (security_barrier)"#,
    ]);
}
