mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_system_parses() {
    run_cases(&["ALTER SYSTEM SET work_mem TO '64MB'"]);
}
