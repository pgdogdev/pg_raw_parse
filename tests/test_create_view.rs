mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_view_parses() {
    run_cases(&["CREATE VIEW my_view AS SELECT 1 AS id"]);
}
