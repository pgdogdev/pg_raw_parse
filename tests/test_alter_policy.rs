mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_policy_parses() {
    run_cases(&["ALTER POLICY my_policy ON my_table RENAME TO my_policy2"]);
}
