mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_conversion_parses() {
    run_cases(&[
        "CREATE DEFAULT CONVERSION my_conversion FOR 'UTF8' TO 'LATIN1' FROM utf8_to_latin1",
    ]);
}
