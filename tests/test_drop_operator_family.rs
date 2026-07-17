mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_operator_family_parses() {
    run_cases(&[
        r#"DROP OPERATOR FAMILY my_family USING btree"#,
        r#"DROP OPERATOR FAMILY IF EXISTS my_family USING btree CASCADE"#,
        r#"DROP OPERATOR FAMILY IF EXISTS my_family USING btree RESTRICT"#,
    ]);
}
