mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_event_trigger_parses() {
    run_cases(&[
        "CREATE EVENT TRIGGER my_event_trigger ON ddl_command_start EXECUTE FUNCTION my_function()",
    ]);
}
