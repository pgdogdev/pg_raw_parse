mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_foreign_data_wrapper_parses() {
    run_cases(&["DROP FOREIGN DATA WRAPPER IF EXISTS my_fdw"]);
}
