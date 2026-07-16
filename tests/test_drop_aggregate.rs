mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_aggregate_parses() {
    run_cases(&["DROP AGGREGATE IF EXISTS my_aggregate(integer)"]);
}
