mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_group_parses() {
    run_cases(&["ALTER GROUP my_group ADD USER my_user"]);
}
