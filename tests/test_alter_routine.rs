mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_routine_parses() {
    run_cases(&["ALTER ROUTINE my_routine(integer) RENAME TO my_routine2"]);
}
