mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_text_search_configuration_parses() {
    run_cases(&["CREATE TEXT SEARCH CONFIGURATION my_config (PARSER = default)"]);
}
