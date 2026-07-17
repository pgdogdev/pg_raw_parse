mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_role_parses() {
    run_cases(&[
        r#"DROP ROLE app_user"#,
        r#"DROP ROLE IF EXISTS app_user, app_user_old"#,
    ]);
}
