mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE POLICY
// Description: define a new row-level security policy for a table
// Syntax:
// CREATE POLICY name ON table_name
//     [ AS { PERMISSIVE | RESTRICTIVE } ]
//     [ FOR { ALL | SELECT | INSERT | UPDATE | DELETE } ]
//     [ TO { role_name | PUBLIC | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] ]
//     [ USING ( using_expression ) ]
//     [ WITH CHECK ( check_expression ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createpolicy.html

#[test]
fn create_policy_parses() {
    run_cases(&[
        r#"CREATE POLICY pol ON my_table"#,
        r#"CREATE POLICY pol ON my_table AS RESTRICTIVE FOR UPDATE TO app_user USING (tenant_id = 1) WITH CHECK (tenant_id = 1)"#,
    ]);
}
