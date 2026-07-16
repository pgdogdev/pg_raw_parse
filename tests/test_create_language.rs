mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_language_parses() {
    run_cases(&["CREATE LANGUAGE my_language"]);
}
