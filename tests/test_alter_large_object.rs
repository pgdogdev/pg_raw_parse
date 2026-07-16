mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_large_object_parses() {
    run_cases(&["ALTER LARGE OBJECT 123 OWNER TO current_user"]);
}
