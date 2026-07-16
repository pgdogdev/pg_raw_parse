mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_tablespace_parses() {
    run_cases(&["CREATE TABLESPACE my_tablespace LOCATION '/tmp/my_tablespace'"]);
}
