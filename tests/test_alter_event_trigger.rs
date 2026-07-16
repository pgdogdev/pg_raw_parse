mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_event_trigger_parses() {
    run_cases(&["ALTER EVENT TRIGGER my_event_trigger RENAME TO my_event_trigger2"]);
}
