mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_group_parses() {
    run_cases(&[
        r#"ALTER GROUP old_group RENAME TO new_group"#,
        r#"ALTER GROUP my_group ADD USER alice, bob"#,
        r#"ALTER GROUP my_group DROP USER alice, bob"#,
    ]);
}
