mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_view_parses() {
    run_cases(&["ALTER VIEW my_view RENAME TO my_view2"]);
}
