mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE USER MAPPING
// Description: define a new mapping of a user to a foreign server
// Syntax:
// CREATE USER MAPPING [ IF NOT EXISTS ] FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | PUBLIC }
//     SERVER server_name
//     [ OPTIONS ( option 'value' [ , ... ] ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createusermapping.html

#[test]
fn create_user_mapping_parses() {
    run_cases(&[
        r#"CREATE USER MAPPING FOR app_user SERVER my_server OPTIONS (user 'remote', password 'secret')"#,
        r#"CREATE USER MAPPING IF NOT EXISTS FOR CURRENT_USER SERVER my_server OPTIONS (user 'current')"#,
        r#"CREATE USER MAPPING FOR PUBLIC SERVER my_server"#,
    ]);
}
