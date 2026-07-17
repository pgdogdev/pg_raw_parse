mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_policy_parses() {
    run_cases(&[
        r#"CREATE POLICY pol ON my_table"#,
        r#"CREATE POLICY pol ON my_table AS RESTRICTIVE FOR UPDATE TO app_user USING (tenant_id = 1) WITH CHECK (tenant_id = 1)"#,
    ]);
}
