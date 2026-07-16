mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_sequence_parses() {
    run_cases(&["ALTER SEQUENCE my_sequence RESTART WITH 1"]);
}
