mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP ROLE
// Description: remove a database role
// Syntax:
// DROP ROLE [ IF EXISTS ] name [, ...]
//
// URL: https://www.postgresql.org/docs/18/sql-droprole.html

#[test]
fn drop_role_parses() {
    run_cases(&[
        r#"DROP ROLE app_user"#,
        r#"DROP ROLE IF EXISTS app_user, app_user_old"#,
    ]);
}
