mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_index_parses() {
    run_cases(&["CREATE INDEX my_index ON my_table (id)"]);
}
