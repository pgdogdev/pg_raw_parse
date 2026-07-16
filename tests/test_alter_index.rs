mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_index_parses() {
    run_cases(&["ALTER INDEX my_index RENAME TO my_index2"]);
}
