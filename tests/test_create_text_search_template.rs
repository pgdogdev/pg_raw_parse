mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_text_search_template_parses() {
    run_cases(&["CREATE TEXT SEARCH TEMPLATE my_template (LEXIZE = dsimple_lexize)"]);
}
