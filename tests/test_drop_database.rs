mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_database_parses() {
    run_cases(&["DROP DATABASE IF EXISTS my_database"]);
}
