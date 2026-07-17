mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_operator_family_parses() {
    run_cases(&[r#"CREATE OPERATOR FAMILY my_family USING btree"#]);
}
