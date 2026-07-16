mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_materialized_view_parses() {
    run_cases(&["DROP MATERIALIZED VIEW IF EXISTS my_matview"]);
}
