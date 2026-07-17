mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP OPERATOR CLASS
// Description: remove an operator class
// Syntax:
// DROP OPERATOR CLASS [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropopclass.html

#[test]
fn drop_operator_class_parses() {
    run_cases(&[
        r#"DROP OPERATOR CLASS my_opclass USING btree"#,
        r#"DROP OPERATOR CLASS IF EXISTS my_opclass USING btree CASCADE"#,
        r#"DROP OPERATOR CLASS IF EXISTS my_opclass USING btree RESTRICT"#,
    ]);
}
