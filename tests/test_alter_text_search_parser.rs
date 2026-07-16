mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_text_search_parser_parses() {
    run_cases(&["ALTER TEXT SEARCH PARSER my_parser RENAME TO my_parser2"]);
}
