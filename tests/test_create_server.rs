mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_server_parses() {
    run_cases(&["CREATE SERVER my_server FOREIGN DATA WRAPPER my_fdw"]);
}
