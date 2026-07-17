mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_operator_parses() {
    run_cases(&[
        r#"CREATE OPERATOR + (FUNCTION = my_add, LEFTARG = integer, RIGHTARG = integer)"#,
        r#"CREATE OPERATOR public.## (PROCEDURE = my_op, LEFTARG = NONE, RIGHTARG = integer, COMMUTATOR = OPERATOR(public.##), NEGATOR = OPERATOR(public.!!), RESTRICT = my_restrict, JOIN = my_join, HASHES, MERGES)"#,
    ]);
}
