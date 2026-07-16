mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn select_into_parses() {
    run_cases(&["SELECT 1 AS id INTO my_table"]);
}
