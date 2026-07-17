mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP USER MAPPING
// Description: remove a user mapping for a foreign server
// Syntax:
// DROP USER MAPPING [ IF EXISTS ] FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | PUBLIC } SERVER server_name
//
// URL: https://www.postgresql.org/docs/18/sql-dropusermapping.html

#[test]
fn drop_user_mapping_parses() {
    run_cases(&[
        r#"DROP USER MAPPING FOR app_user SERVER my_server"#,
        r#"DROP USER MAPPING IF EXISTS FOR CURRENT_USER SERVER my_server"#,
        r#"DROP USER MAPPING IF EXISTS FOR PUBLIC SERVER my_server"#,
    ]);
}
