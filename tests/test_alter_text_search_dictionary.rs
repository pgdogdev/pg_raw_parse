mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_text_search_dictionary_parses() {
    run_cases(&["ALTER TEXT SEARCH DICTIONARY my_dictionary RENAME TO my_dictionary2"]);
}
