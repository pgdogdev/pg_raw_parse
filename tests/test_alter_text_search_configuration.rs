mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_text_search_configuration_parses() {
    run_cases(&["ALTER TEXT SEARCH CONFIGURATION my_config RENAME TO my_config2"]);
}
