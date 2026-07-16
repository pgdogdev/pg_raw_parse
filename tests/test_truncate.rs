mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn truncate_parses() {
    run_cases(&["TRUNCATE TABLE my_table"]);
}
