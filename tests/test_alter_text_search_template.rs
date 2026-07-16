mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_text_search_template_parses() {
    run_cases(&["ALTER TEXT SEARCH TEMPLATE my_template RENAME TO my_template2"]);
}
