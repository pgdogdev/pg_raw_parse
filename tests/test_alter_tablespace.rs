mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TABLESPACE
// Description: change the definition of a tablespace
// Syntax:
// ALTER TABLESPACE name RENAME TO new_name
// ALTER TABLESPACE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TABLESPACE name SET ( tablespace_option = value [, ... ] )
// ALTER TABLESPACE name RESET ( tablespace_option [, ... ] )
//
// URL: https://www.postgresql.org/docs/18/sql-altertablespace.html

#[test]
fn alter_tablespace_parses() {
    run_cases(&[
        r#"ALTER TABLESPACE fastspace RENAME TO slowspace"#,
        r#"ALTER TABLESPACE fastspace OWNER TO CURRENT_USER"#,
        r#"ALTER TABLESPACE fastspace SET (random_page_cost = 1.1, seq_page_cost = 1.0)"#,
        r#"ALTER TABLESPACE fastspace RESET (random_page_cost)"#,
    ]);
}
