mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER POLICY
// Description: change the definition of a row-level security policy
// Syntax:
// ALTER POLICY name ON table_name RENAME TO new_name
//
// ALTER POLICY name ON table_name
//     [ TO { role_name | PUBLIC | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] ]
//     [ USING ( using_expression ) ]
//     [ WITH CHECK ( check_expression ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-alterpolicy.html

#[test]
fn alter_policy_parses() {
    run_cases(&[
        r#"ALTER POLICY pol ON my_table RENAME TO pol_new"#,
        r#"ALTER POLICY pol ON my_table TO app_user USING (tenant_id = 1) WITH CHECK (tenant_id = 1)"#,
    ]);
}
