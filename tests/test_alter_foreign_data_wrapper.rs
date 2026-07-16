mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_foreign_data_wrapper_parses() {
    run_cases(&["ALTER FOREIGN DATA WRAPPER my_fdw RENAME TO my_fdw2"]);
}
