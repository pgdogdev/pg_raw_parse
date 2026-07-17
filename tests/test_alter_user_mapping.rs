mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_user_mapping_parses() {
    run_cases(&[
        r#"ALTER USER MAPPING FOR app_user SERVER my_server OPTIONS (ADD user 'remote', SET password 'secret')"#,
        r#"ALTER USER MAPPING FOR CURRENT_USER SERVER my_server OPTIONS (DROP password)"#,
        r#"ALTER USER MAPPING FOR PUBLIC SERVER my_server OPTIONS (SET user 'public')"#,
    ]);
}
