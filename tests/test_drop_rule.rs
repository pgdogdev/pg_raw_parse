mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_rule_parses() {
    run_cases(&[
        r#"DROP RULE my_rule ON my_table"#,
        r#"DROP RULE IF EXISTS my_rule ON my_table CASCADE"#,
        r#"DROP RULE IF EXISTS my_rule ON my_table RESTRICT"#,
    ]);
}
