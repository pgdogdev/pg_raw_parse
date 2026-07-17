mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_rule_parses() {
    run_cases(&[r#"ALTER RULE my_rule ON my_table RENAME TO my_rule_new"#]);
}
