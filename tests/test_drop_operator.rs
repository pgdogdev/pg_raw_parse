mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP OPERATOR
// Description: remove an operator
// Syntax:
// DROP OPERATOR [ IF EXISTS ] name ( { left_type | NONE } , right_type ) [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropoperator.html

#[test]
fn drop_operator_parses() {
    run_cases(&[
        r#"DROP OPERATOR +(integer, integer)"#,
        r#"DROP OPERATOR IF EXISTS +(integer, integer), -(integer, integer) CASCADE"#,
    ]);
}
