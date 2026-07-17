mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TABLESPACE
// Description: define a new tablespace
// Syntax:
// CREATE TABLESPACE tablespace_name
//     [ OWNER { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER } ]
//     LOCATION 'directory'
//     [ WITH ( tablespace_option = value [, ... ] ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createtablespace.html

#[test]
fn create_tablespace_parses() {
    run_cases(&[
        r#"CREATE TABLESPACE fastspace LOCATION '/tmp/fastspace'"#,
        r#"CREATE TABLESPACE fastspace OWNER app_user LOCATION '/tmp/fastspace' WITH (random_page_cost = 1.1)"#,
    ]);
}
