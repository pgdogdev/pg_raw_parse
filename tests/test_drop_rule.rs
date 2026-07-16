mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_rule_parses() {
    run_cases(&["DROP RULE IF EXISTS my_rule ON my_table"]);
}
