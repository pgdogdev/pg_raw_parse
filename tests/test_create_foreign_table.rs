mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_foreign_table_parses() {
    run_cases(&["CREATE FOREIGN TABLE my_foreign_table (id integer) SERVER my_server"]);
}
