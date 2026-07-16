mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_procedure_parses() {
    run_cases(&["ALTER PROCEDURE my_procedure(integer) RENAME TO my_procedure2"]);
}
