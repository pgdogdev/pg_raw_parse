mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_server_parses() {
    run_cases(&["DROP SERVER IF EXISTS my_server"]);
}
