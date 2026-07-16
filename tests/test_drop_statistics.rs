mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_statistics_parses() {
    run_cases(&["DROP STATISTICS IF EXISTS my_statistics"]);
}
