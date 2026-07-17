mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER RULE
// Description: change the definition of a rule
// Syntax:
// ALTER RULE name ON table_name RENAME TO new_name
//
// URL: https://www.postgresql.org/docs/18/sql-alterrule.html

#[test]
fn alter_rule_parses() {
    run_cases(&[r#"ALTER RULE my_rule ON my_table RENAME TO my_rule_new"#]);
}
