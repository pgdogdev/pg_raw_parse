mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_text_search_dictionary_parses() {
    run_cases(&["DROP TEXT SEARCH DICTIONARY IF EXISTS my_dictionary"]);
}
