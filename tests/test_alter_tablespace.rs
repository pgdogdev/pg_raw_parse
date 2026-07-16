mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_tablespace_parses() {
    run_cases(&["ALTER TABLESPACE my_tablespace RENAME TO my_tablespace2"]);
}
