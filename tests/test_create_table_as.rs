mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_table_as_parses() {
    run_cases(&["CREATE TABLE my_table AS SELECT 1 AS id"]);
}
