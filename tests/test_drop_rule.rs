mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP RULE
// Description: remove a rewrite rule
// Syntax:
// DROP RULE [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droprule.html

#[test]
fn drop_rule_parses() {
    run_cases(&[
        r#"DROP RULE my_rule ON my_table"#,
        r#"DROP RULE IF EXISTS my_rule ON my_table CASCADE"#,
        r#"DROP RULE IF EXISTS my_rule ON my_table RESTRICT"#,
    ]);
}
