mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP EVENT TRIGGER
// Description: remove an event trigger
// Syntax:
// DROP EVENT TRIGGER [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropeventtrigger.html

#[test]
fn drop_event_trigger_parses() {
    run_cases(&[
        r#"DROP EVENT TRIGGER trg"#,
        r#"DROP EVENT TRIGGER IF EXISTS trg CASCADE"#,
    ]);
}
