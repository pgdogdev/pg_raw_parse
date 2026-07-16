mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_user_mapping_parses() {
    run_cases(&[
        "ALTER USER MAPPING FOR my_user SERVER my_server OPTIONS (SET user 'remote_user')",
    ]);
}
