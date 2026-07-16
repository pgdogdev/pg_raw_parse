mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_sequence_parses() {
    run_cases(&["DROP SEQUENCE IF EXISTS my_sequence"]);
}
