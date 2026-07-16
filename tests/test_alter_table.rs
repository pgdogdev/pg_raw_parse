mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_table_parses() {
    run_cases(&["ALTER TABLE my_table ADD COLUMN new_column integer"]);
}
