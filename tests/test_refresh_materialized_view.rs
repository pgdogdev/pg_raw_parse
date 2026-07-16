mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn refresh_materialized_view_parses() {
    run_cases(&["REFRESH MATERIALIZED VIEW my_matview"]);
}
