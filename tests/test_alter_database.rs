mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_database_parses() {
    run_cases(&["ALTER DATABASE mydb SET work_mem TO '64MB'"]);
}
