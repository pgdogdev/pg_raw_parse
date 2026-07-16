mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_operator_family_parses() {
    run_cases(&["ALTER OPERATOR FAMILY my_opfamily USING btree RENAME TO my_opfamily2"]);
}
