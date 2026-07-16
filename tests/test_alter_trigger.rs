mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_trigger_parses() {
    run_cases(&["ALTER TRIGGER my_trigger ON my_table RENAME TO my_trigger2"]);
}
