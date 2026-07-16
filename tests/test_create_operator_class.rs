mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_operator_class_parses() {
    run_cases(&[
        "CREATE OPERATOR CLASS my_opclass DEFAULT FOR TYPE integer USING btree AS OPERATOR 1 < (integer, integer), FUNCTION 1 btint4cmp(integer, integer)",
    ]);
}
