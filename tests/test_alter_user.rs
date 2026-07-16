mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_user_parses() {
    run_cases(&["ALTER USER my_user SET work_mem TO '64MB'"]);
}
