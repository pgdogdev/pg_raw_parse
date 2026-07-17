mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER CONVERSION
// Description: change the definition of a conversion
// Syntax:
// ALTER CONVERSION name RENAME TO new_name
// ALTER CONVERSION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER CONVERSION name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-alterconversion.html

#[test]
fn alter_conversion_parses() {
    run_cases(&[
        r#"ALTER CONVERSION my_conversion RENAME TO my_new_conversion"#,
        r#"ALTER CONVERSION my_conversion OWNER TO CURRENT_USER"#,
        r#"ALTER CONVERSION my_conversion SET SCHEMA public"#,
    ]);
}
