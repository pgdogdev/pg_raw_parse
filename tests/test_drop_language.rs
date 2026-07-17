mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_language_parses() {
    run_cases(&[
        r#"DROP LANGUAGE plsample"#,
        r#"DROP LANGUAGE IF EXISTS plsample, plsample_old CASCADE"#,
        r#"DROP LANGUAGE IF EXISTS plsample RESTRICT"#,
    ]);
}
