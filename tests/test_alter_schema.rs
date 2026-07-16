mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_schema_parses() {
    run_cases(&["ALTER SCHEMA my_schema RENAME TO my_schema2"]);
}
