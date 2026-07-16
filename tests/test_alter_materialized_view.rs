mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_materialized_view_parses() {
    run_cases(&["ALTER MATERIALIZED VIEW my_matview RENAME TO my_matview2"]);
}
