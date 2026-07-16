mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_role_parses() {
    run_cases(&["ALTER ROLE my_role SET work_mem TO '64MB'"]);
}
