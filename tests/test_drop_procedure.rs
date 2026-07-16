mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_procedure_parses() {
    run_cases(&["DROP PROCEDURE IF EXISTS my_procedure(integer)"]);
}
