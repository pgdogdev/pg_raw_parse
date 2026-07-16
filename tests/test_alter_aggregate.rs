mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_aggregate_parses() {
    run_cases(&["ALTER AGGREGATE my_sum(integer) RENAME TO my_sum2"]);
}
