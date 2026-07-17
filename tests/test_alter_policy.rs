mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_policy_parses() {
    run_cases(&[
        r#"ALTER POLICY pol ON my_table RENAME TO pol_new"#,
        r#"ALTER POLICY pol ON my_table TO app_user USING (tenant_id = 1) WITH CHECK (tenant_id = 1)"#,
    ]);
}
