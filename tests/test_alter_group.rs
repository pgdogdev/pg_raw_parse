mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER GROUP
// Description: change role name or membership
// Syntax:
// ALTER GROUP role_specification ADD USER user_name [, ... ]
// ALTER GROUP role_specification DROP USER user_name [, ... ]
//
// where role_specification can be:
//
//     role_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
//
// ALTER GROUP group_name RENAME TO new_name
//
// URL: https://www.postgresql.org/docs/18/sql-altergroup.html

#[test]
fn alter_group_parses() {
    run_cases(&[
        r#"ALTER GROUP old_group RENAME TO new_group"#,
        r#"ALTER GROUP my_group ADD USER alice, bob"#,
        r#"ALTER GROUP my_group DROP USER alice, bob"#,
    ]);
}
