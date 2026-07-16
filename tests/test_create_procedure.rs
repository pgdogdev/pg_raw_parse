mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_procedure_parses() {
    run_cases(&["CREATE PROCEDURE my_procedure() LANGUAGE SQL AS 'SELECT 1'"]);
}
