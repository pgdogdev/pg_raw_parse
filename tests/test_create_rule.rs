mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE RULE
// Description: define a new rewrite rule
// Syntax:
// CREATE [ OR REPLACE ] RULE name AS ON event
//     TO table_name [ WHERE condition ]
//     DO [ ALSO | INSTEAD ] { NOTHING | command | ( command ; command ... ) }
//
// where event can be one of:
//
//     SELECT | INSERT | UPDATE | DELETE
//
// URL: https://www.postgresql.org/docs/18/sql-createrule.html

#[test]
fn create_rule_parses() {
    run_cases(&[
        r#"CREATE RULE my_rule AS ON INSERT TO my_table DO INSTEAD NOTHING"#,
        r#"CREATE OR REPLACE RULE my_rule AS ON UPDATE TO my_table WHERE NEW.id > 0 DO ALSO (NOTIFY my_channel; SELECT 1)"#,
    ]);
}
