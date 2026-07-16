mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_text_search_dictionary_parses() {
    run_cases(&["CREATE TEXT SEARCH DICTIONARY my_dictionary (TEMPLATE = simple)"]);
}
