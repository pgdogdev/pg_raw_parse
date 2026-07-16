mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_extension_parses() {
    run_cases(&["ALTER EXTENSION hstore UPDATE"]);
}
