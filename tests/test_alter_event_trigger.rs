mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_event_trigger_parses() {
    run_cases(&[
        r#"ALTER EVENT TRIGGER trg DISABLE"#,
        r#"ALTER EVENT TRIGGER trg ENABLE"#,
        r#"ALTER EVENT TRIGGER trg ENABLE REPLICA"#,
        r#"ALTER EVENT TRIGGER trg ENABLE ALWAYS"#,
        r#"ALTER EVENT TRIGGER trg OWNER TO CURRENT_USER"#,
        r#"ALTER EVENT TRIGGER trg RENAME TO trg_new"#,
    ]);
}
