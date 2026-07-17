mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_transform_parses() {
    run_cases(&[
        r#"DROP TRANSFORM FOR my_type LANGUAGE plsample"#,
        r#"DROP TRANSFORM IF EXISTS FOR my_type LANGUAGE plsample CASCADE"#,
    ]);
}
