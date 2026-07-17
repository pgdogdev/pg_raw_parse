mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE OPERATOR
// Description: define a new operator
// Syntax:
// CREATE OPERATOR name (
//     {FUNCTION|PROCEDURE} = function_name
//     [, LEFTARG = left_type ] [, RIGHTARG = right_type ]
//     [, COMMUTATOR = com_op ] [, NEGATOR = neg_op ]
//     [, RESTRICT = res_proc ] [, JOIN = join_proc ]
//     [, HASHES ] [, MERGES ]
// )
//
// URL: https://www.postgresql.org/docs/18/sql-createoperator.html

#[test]
fn create_operator_parses() {
    run_cases(&[
        r#"CREATE OPERATOR + (FUNCTION = my_add, LEFTARG = integer, RIGHTARG = integer)"#,
        r#"CREATE OPERATOR public.## (PROCEDURE = my_op, LEFTARG = NONE, RIGHTARG = integer, COMMUTATOR = OPERATOR(public.##), NEGATOR = OPERATOR(public.!!), RESTRICT = my_restrict, JOIN = my_join, HASHES, MERGES)"#,
    ]);
}
