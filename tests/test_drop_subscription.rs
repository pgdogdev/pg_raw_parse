mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_subscription_parses() {
    run_cases(&[
        r#"DROP SUBSCRIPTION sub"#,
        r#"DROP SUBSCRIPTION IF EXISTS sub CASCADE"#,
        r#"DROP SUBSCRIPTION IF EXISTS sub RESTRICT"#,
    ]);
}
