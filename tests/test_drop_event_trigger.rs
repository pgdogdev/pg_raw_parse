mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_event_trigger_parses() {
    run_cases(&["DROP EVENT TRIGGER IF EXISTS my_event_trigger"]);
}
