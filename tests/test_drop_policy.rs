mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP POLICY
// Description: remove a row-level security policy from a table
// Syntax:
// DROP POLICY [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droppolicy.html

#[test]
fn drop_policy_parses() {
    run_cases(&[
        r#"DROP POLICY pol ON my_table"#,
        r#"DROP POLICY IF EXISTS pol ON my_table"#,
    ]);
}
