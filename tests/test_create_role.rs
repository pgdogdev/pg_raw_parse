mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_role_parses() {
    run_cases(&["CREATE ROLE my_role"]);
}
