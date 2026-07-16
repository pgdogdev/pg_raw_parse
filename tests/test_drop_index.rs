mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_index_parses() {
    run_cases(&["DROP INDEX IF EXISTS my_index"]);
}
