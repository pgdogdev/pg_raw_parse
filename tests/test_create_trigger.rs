mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_trigger_parses() {
    run_cases(&[
        r#"CREATE TRIGGER trg BEFORE INSERT OR UPDATE OF name ON my_table FOR EACH ROW WHEN (NEW.id > 0) EXECUTE FUNCTION my_function()"#,
        r#"CREATE OR REPLACE TRIGGER trg AFTER DELETE ON my_table FOR EACH ROW EXECUTE FUNCTION my_function()"#,
        r#"CREATE CONSTRAINT TRIGGER trg AFTER DELETE ON my_table DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION my_function()"#,
        r#"CREATE TRIGGER trg AFTER TRUNCATE ON my_table REFERENCING OLD TABLE AS old_table NEW TABLE AS new_table FOR EACH STATEMENT EXECUTE PROCEDURE my_function()"#,
    ]);
}
