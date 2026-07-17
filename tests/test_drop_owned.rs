mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP OWNED
// Description: remove database objects owned by a database role
// Syntax:
// DROP OWNED BY { name | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-drop-owned.html

#[test]
fn drop_owned_parses() {
    run_cases(&[
        r#"DROP OWNED BY app_user"#,
        r#"DROP OWNED BY app_user, other_user CASCADE"#,
    ]);
}
