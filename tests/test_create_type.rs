mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_type_parses() {
    run_cases(&["CREATE TYPE my_type AS (id integer)"]);
}
