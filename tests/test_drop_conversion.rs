mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_conversion_parses() {
    run_cases(&["DROP CONVERSION IF EXISTS my_conversion"]);
}
