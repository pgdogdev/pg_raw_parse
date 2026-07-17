mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_procedure_parses() {
    run_cases(&[
        r#"DROP PROCEDURE my_proc(integer)"#,
        r#"DROP PROCEDURE IF EXISTS my_proc(integer), my_proc(text) CASCADE"#,
    ]);
}
