mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_extension_parses() {
    run_cases(&["DROP EXTENSION IF EXISTS hstore"]);
}
