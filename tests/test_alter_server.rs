mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER SERVER
// Description: change the definition of a foreign server
// Syntax:
// ALTER SERVER name [ VERSION 'new_version' ]
//     [ OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ] ) ]
// ALTER SERVER name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER SERVER name RENAME TO new_name
//
// URL: https://www.postgresql.org/docs/18/sql-alterserver.html

#[test]
fn alter_server_parses() {
    run_cases(&[
        r#"ALTER SERVER my_server VERSION '2.0' OPTIONS (ADD host 'localhost', SET port '5432', DROP oldopt)"#,
        r#"ALTER SERVER my_server OWNER TO CURRENT_USER"#,
        r#"ALTER SERVER my_server RENAME TO my_server_new"#,
    ]);
}
