mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_type_parses() {
    run_cases(&["ALTER TYPE my_type RENAME TO my_type2"]);
}
