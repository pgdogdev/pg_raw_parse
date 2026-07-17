mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER USER MAPPING
// Description: change the definition of a user mapping
// Syntax:
// ALTER USER MAPPING FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | SESSION_USER | PUBLIC }
//     SERVER server_name
//     OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ] )
//
// URL: https://www.postgresql.org/docs/18/sql-alterusermapping.html

#[test]
fn alter_user_mapping_parses() {
    run_cases(&[
        r#"ALTER USER MAPPING FOR app_user SERVER my_server OPTIONS (ADD user 'remote', SET password 'secret')"#,
        r#"ALTER USER MAPPING FOR CURRENT_USER SERVER my_server OPTIONS (DROP password)"#,
        r#"ALTER USER MAPPING FOR PUBLIC SERVER my_server OPTIONS (SET user 'public')"#,
    ]);
}
