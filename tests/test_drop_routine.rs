mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_routine_parses() {
    run_cases(&[
        r#"DROP ROUTINE my_function(integer)"#,
        r#"DROP ROUTINE IF EXISTS my_function(integer), my_proc(text) CASCADE"#,
    ]);
}
