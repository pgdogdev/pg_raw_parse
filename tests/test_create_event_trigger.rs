mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE EVENT TRIGGER
// Description: define a new event trigger
// Syntax:
// CREATE EVENT TRIGGER name
//     ON event
//     [ WHEN filter_variable IN (filter_value [, ... ]) [ AND ... ] ]
//     EXECUTE { FUNCTION | PROCEDURE } function_name()
//
// URL: https://www.postgresql.org/docs/18/sql-createeventtrigger.html

#[test]
fn create_event_trigger_parses() {
    run_cases(&[
        r#"CREATE EVENT TRIGGER trg ON ddl_command_start EXECUTE FUNCTION my_function()"#,
        r#"CREATE EVENT TRIGGER trg ON sql_drop WHEN TAG IN ('DROP TABLE', 'DROP VIEW') EXECUTE FUNCTION my_function()"#,
    ]);
}
