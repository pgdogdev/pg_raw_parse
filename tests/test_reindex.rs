mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn reindex_parses() {
    run_cases(&["REINDEX INDEX my_index"]);
}
