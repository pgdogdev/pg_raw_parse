mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_policy_parses() {
    run_cases(&["CREATE POLICY my_policy ON my_table USING (true)"]);
}
