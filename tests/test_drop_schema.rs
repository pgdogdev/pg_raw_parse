mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_schema_parses() {
    run_cases(&["DROP SCHEMA IF EXISTS my_schema"]);
}
