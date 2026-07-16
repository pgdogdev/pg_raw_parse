mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_function_parses() {
    run_cases(&["ALTER FUNCTION my_function(integer) RENAME TO my_function2"]);
}
