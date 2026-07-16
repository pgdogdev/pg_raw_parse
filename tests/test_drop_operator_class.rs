mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_operator_class_parses() {
    run_cases(&["DROP OPERATOR CLASS IF EXISTS my_opclass USING btree"]);
}
