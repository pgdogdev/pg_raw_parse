mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_conversion_parses() {
    run_cases(&[
        r#"DROP CONVERSION my_conversion"#,
        r#"DROP CONVERSION IF EXISTS my_conversion, my_conversion_old CASCADE"#,
        r#"DROP CONVERSION IF EXISTS my_conversion RESTRICT"#,
    ]);
}
