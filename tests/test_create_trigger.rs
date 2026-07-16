mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_trigger_parses() {
    run_cases(&[
        "CREATE TRIGGER my_trigger BEFORE INSERT ON my_table FOR EACH ROW EXECUTE FUNCTION my_function()",
    ]);
}
