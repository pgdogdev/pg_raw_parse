mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_foreign_table_parses() {
    run_cases(&["DROP FOREIGN TABLE IF EXISTS my_foreign_table"]);
}
