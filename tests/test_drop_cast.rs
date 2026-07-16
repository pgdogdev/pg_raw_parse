mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_cast_parses() {
    run_cases(&["DROP CAST IF EXISTS (integer AS bigint)"]);
}
