mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_server_parses() {
    run_cases(&["ALTER SERVER my_server RENAME TO my_server2"]);
}
