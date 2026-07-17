mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_large_object_parses() {
    run_cases(&[r#"ALTER LARGE OBJECT 12345 OWNER TO CURRENT_USER"#]);
}
