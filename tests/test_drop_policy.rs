mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_policy_parses() {
    run_cases(&["DROP POLICY IF EXISTS my_policy ON my_table"]);
}
