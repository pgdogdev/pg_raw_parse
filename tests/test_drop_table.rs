mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_table_parses() {
    run_cases(&["DROP TABLE IF EXISTS my_table"]);
}
