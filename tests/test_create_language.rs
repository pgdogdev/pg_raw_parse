mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_language_parses() {
    run_cases(&[
        r#"CREATE LANGUAGE plsample"#,
        r#"CREATE OR REPLACE TRUSTED PROCEDURAL LANGUAGE plsample HANDLER my_handler INLINE my_inline VALIDATOR my_validator"#,
    ]);
}
