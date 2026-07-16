mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_operator_parses() {
    run_cases(&["CREATE OPERATOR === (FUNCTION = int4eq, LEFTARG = integer, RIGHTARG = integer)"]);
}
