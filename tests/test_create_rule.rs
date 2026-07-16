mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_rule_parses() {
    run_cases(&["CREATE RULE my_rule AS ON SELECT TO my_table DO INSTEAD SELECT * FROM my_table"]);
}
