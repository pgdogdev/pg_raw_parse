mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_statistics_parses() {
    run_cases(&["ALTER STATISTICS my_statistics SET STATISTICS 100"]);
}
