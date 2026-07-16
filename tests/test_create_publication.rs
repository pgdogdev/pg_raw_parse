mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_publication_parses() {
    run_cases(&["CREATE PUBLICATION my_publication FOR TABLE my_table"]);
}
