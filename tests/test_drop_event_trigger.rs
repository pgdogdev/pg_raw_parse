mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_event_trigger_parses() {
    run_cases(&[
        r#"DROP EVENT TRIGGER trg"#,
        r#"DROP EVENT TRIGGER IF EXISTS trg CASCADE"#,
    ]);
}
