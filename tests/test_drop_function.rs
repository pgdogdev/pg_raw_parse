mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_function_parses() {
    run_cases(&[
        r#"DROP FUNCTION my_function(integer)"#,
        r#"DROP FUNCTION IF EXISTS my_function(integer), my_function(text) CASCADE"#,
    ]);
}
