mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_cast_parses() {
    run_cases(&["CREATE CAST (integer AS bigint) WITHOUT FUNCTION AS IMPLICIT"]);
}
