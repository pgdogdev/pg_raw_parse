mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_conversion_parses() {
    run_cases(&[
        r#"CREATE DEFAULT CONVERSION my_conversion FOR 'UTF8' TO 'LATIN1' FROM my_convert_function"#,
        r#"CREATE CONVERSION my_conversion FOR 'LATIN1' TO 'UTF8' FROM my_convert_function"#,
    ]);
}
