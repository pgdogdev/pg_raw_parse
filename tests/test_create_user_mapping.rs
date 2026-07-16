mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_user_mapping_parses() {
    run_cases(&["CREATE USER MAPPING FOR my_user SERVER my_server OPTIONS (user 'remote_user')"]);
}
