mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_language_parses() {
    run_cases(&["DROP LANGUAGE IF EXISTS my_language"]);
}
