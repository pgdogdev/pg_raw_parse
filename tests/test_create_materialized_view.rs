mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_materialized_view_parses() {
    run_cases(&["CREATE MATERIALIZED VIEW my_matview AS SELECT 1 AS id"]);
}
