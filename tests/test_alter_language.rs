mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_language_parses() {
    run_cases(&["ALTER LANGUAGE my_language RENAME TO my_language2"]);
}
