mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TRIGGER
// Description: define a new trigger
// Syntax:
// CREATE [ OR REPLACE ] [ CONSTRAINT ] TRIGGER name { BEFORE | AFTER | INSTEAD OF } { event [ OR ... ] }
//     ON table_name
//     [ FROM referenced_table_name ]
//     [ NOT DEFERRABLE | [ DEFERRABLE ] [ INITIALLY IMMEDIATE | INITIALLY DEFERRED ] ]
//     [ REFERENCING { { OLD | NEW } TABLE [ AS ] transition_relation_name } [ ... ] ]
//     [ FOR [ EACH ] { ROW | STATEMENT } ]
//     [ WHEN ( condition ) ]
//     EXECUTE { FUNCTION | PROCEDURE } function_name ( arguments )
//
// where event can be one of:
//
//     INSERT
//     UPDATE [ OF column_name [, ... ] ]
//     DELETE
//     TRUNCATE
//
// URL: https://www.postgresql.org/docs/18/sql-createtrigger.html

#[test]
fn create_trigger_parses() {
    run_cases(&[
        r#"CREATE TRIGGER trg BEFORE INSERT OR UPDATE OF name ON my_table FOR EACH ROW WHEN (NEW.id > 0) EXECUTE FUNCTION my_function()"#,
        r#"CREATE OR REPLACE TRIGGER trg AFTER DELETE ON my_table FOR EACH ROW EXECUTE FUNCTION my_function()"#,
        r#"CREATE CONSTRAINT TRIGGER trg AFTER DELETE ON my_table DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION my_function()"#,
        r#"CREATE TRIGGER trg AFTER TRUNCATE ON my_table REFERENCING OLD TABLE AS old_table NEW TABLE AS new_table FOR EACH STATEMENT EXECUTE PROCEDURE my_function()"#,
    ]);
}
