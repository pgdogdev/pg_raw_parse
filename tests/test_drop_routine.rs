mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_routine_parses() {
    run_cases(&["DROP ROUTINE IF EXISTS my_routine(integer)"]);
}
