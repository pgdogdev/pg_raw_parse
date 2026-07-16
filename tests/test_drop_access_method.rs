mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_access_method_parses() {
    run_cases(&["DROP ACCESS METHOD IF EXISTS my_am"]);
}
