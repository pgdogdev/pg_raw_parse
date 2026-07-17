mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_trigger_parses() {
    run_cases(&[
        r#"DROP TRIGGER trg ON my_table"#,
        r#"DROP TRIGGER IF EXISTS trg ON my_table CASCADE"#,
    ]);
}
