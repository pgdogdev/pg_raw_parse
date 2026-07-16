mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_group_parses() {
    run_cases(&["DROP GROUP IF EXISTS my_group"]);
}
