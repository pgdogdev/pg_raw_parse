mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_collation_parses() {
    run_cases(&["ALTER COLLATION my_collation RENAME TO my_collation2"]);
}
