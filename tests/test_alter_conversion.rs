mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_conversion_parses() {
    run_cases(&["ALTER CONVERSION my_conversion RENAME TO my_conversion2"]);
}
