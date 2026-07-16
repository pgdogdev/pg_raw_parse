mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_type_parses() {
    run_cases(&["DROP TYPE IF EXISTS my_type"]);
}
