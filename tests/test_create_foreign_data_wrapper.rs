mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_foreign_data_wrapper_parses() {
    run_cases(&["CREATE FOREIGN DATA WRAPPER my_fdw HANDLER my_fdw_handler"]);
}
