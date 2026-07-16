mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_aggregate_parses() {
    run_cases(&["CREATE AGGREGATE my_aggregate(integer) (SFUNC = int4pl, STYPE = integer)"]);
}
