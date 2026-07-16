mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_database_parses() {
    run_cases(&["CREATE DATABASE my_database"]);
}
