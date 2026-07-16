mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_trigger_parses() {
    run_cases(&["DROP TRIGGER IF EXISTS my_trigger ON my_table"]);
}
