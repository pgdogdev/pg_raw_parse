mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_operator_parses() {
    run_cases(&["DROP OPERATOR IF EXISTS + (integer, integer)"]);
}
