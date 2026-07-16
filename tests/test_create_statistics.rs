mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_statistics_parses() {
    run_cases(&["CREATE STATISTICS my_statistics ON a, b FROM my_table"]);
}
