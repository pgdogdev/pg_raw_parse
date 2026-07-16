mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_text_search_template_parses() {
    run_cases(&["DROP TEXT SEARCH TEMPLATE IF EXISTS my_template"]);
}
