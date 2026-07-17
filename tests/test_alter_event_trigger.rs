mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER EVENT TRIGGER
// Description: change the definition of an event trigger
// Syntax:
// ALTER EVENT TRIGGER name DISABLE
// ALTER EVENT TRIGGER name ENABLE [ REPLICA | ALWAYS ]
// ALTER EVENT TRIGGER name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER EVENT TRIGGER name RENAME TO new_name
//
// URL: https://www.postgresql.org/docs/18/sql-altereventtrigger.html

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
