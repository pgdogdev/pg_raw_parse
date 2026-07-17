mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE SERVER
// Description: define a new foreign server
// Syntax:
// CREATE SERVER [ IF NOT EXISTS ] server_name [ TYPE 'server_type' ] [ VERSION 'server_version' ]
//     FOREIGN DATA WRAPPER fdw_name
//     [ OPTIONS ( option 'value' [, ... ] ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createserver.html

#[test]
fn create_server_parses() {
    run_cases(&[
        r#"CREATE SERVER my_server FOREIGN DATA WRAPPER my_fdw"#,
        r#"CREATE SERVER IF NOT EXISTS my_server TYPE 'postgres' VERSION '18' FOREIGN DATA WRAPPER my_fdw OPTIONS (host 'localhost', dbname 'postgres')"#,
    ]);
}
