mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_view_parses() {
    run_cases(&["DROP VIEW IF EXISTS my_view"]);
}
