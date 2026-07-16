mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_extension_parses() {
    run_cases(&["CREATE EXTENSION IF NOT EXISTS hstore"]);
}
