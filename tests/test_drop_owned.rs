mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_owned_parses() {
    run_cases(&["DROP OWNED BY my_role"]);
}
