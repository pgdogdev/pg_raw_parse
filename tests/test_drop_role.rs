mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_role_parses() {
    run_cases(&["DROP ROLE IF EXISTS my_role"]);
}
