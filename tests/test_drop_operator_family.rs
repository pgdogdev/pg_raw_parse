mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP OPERATOR FAMILY
// Description: remove an operator family
// Syntax:
// DROP OPERATOR FAMILY [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropopfamily.html

#[test]
fn drop_operator_family_parses() {
    run_cases(&[
        r#"DROP OPERATOR FAMILY my_family USING btree"#,
        r#"DROP OPERATOR FAMILY IF EXISTS my_family USING btree CASCADE"#,
        r#"DROP OPERATOR FAMILY IF EXISTS my_family USING btree RESTRICT"#,
    ]);
}
