mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_rule_parses() {
    run_cases(&[
        r#"CREATE RULE my_rule AS ON INSERT TO my_table DO INSTEAD NOTHING"#,
        r#"CREATE OR REPLACE RULE my_rule AS ON UPDATE TO my_table WHERE NEW.id > 0 DO ALSO (NOTIFY my_channel; SELECT 1)"#,
    ]);
}
