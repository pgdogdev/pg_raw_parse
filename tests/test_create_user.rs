mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_user_parses() {
    run_cases(&["CREATE USER my_user"]);
}
