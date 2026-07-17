mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_owned_parses() {
    run_cases(&[
        r#"DROP OWNED BY app_user"#,
        r#"DROP OWNED BY app_user, other_user CASCADE"#,
    ]);
}
