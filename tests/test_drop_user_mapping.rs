mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_user_mapping_parses() {
    run_cases(&["DROP USER MAPPING IF EXISTS FOR my_user SERVER my_server"]);
}
