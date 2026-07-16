mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_operator_parses() {
    run_cases(&["ALTER OPERATOR + (integer, integer) OWNER TO current_user"]);
}
