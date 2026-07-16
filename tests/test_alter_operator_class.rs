mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_operator_class_parses() {
    run_cases(&["ALTER OPERATOR CLASS my_opclass USING btree RENAME TO my_opclass2"]);
}
