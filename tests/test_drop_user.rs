mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_user_parses() {
    run_cases(&["DROP USER IF EXISTS my_user"]);
}
