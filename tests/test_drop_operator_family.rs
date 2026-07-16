mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_operator_family_parses() {
    run_cases(&["DROP OPERATOR FAMILY IF EXISTS my_opfamily USING btree"]);
}
