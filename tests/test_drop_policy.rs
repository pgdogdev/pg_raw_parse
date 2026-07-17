mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_policy_parses() {
    run_cases(&[
        r#"DROP POLICY pol ON my_table"#,
        r#"DROP POLICY IF EXISTS pol ON my_table"#,
    ]);
}
