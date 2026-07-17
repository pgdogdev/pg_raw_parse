mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_user_parses() {
    run_cases(&[
        r#"DROP USER app_user"#,
        r#"DROP USER IF EXISTS app_user, app_user_old"#,
    ]);
}
