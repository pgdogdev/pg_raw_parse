mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_collation_parses() {
    run_cases(&["DROP COLLATION IF EXISTS my_collation"]);
}
