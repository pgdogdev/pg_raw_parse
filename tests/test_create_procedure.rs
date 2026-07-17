mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_procedure_parses() {
    run_cases(&[
        r#"CREATE PROCEDURE my_proc() LANGUAGE SQL AS 'SELECT 1'"#,
        r#"CREATE OR REPLACE PROCEDURE my_proc(IN arg integer DEFAULT 1, OUT value integer) LANGUAGE SQL SECURITY DEFINER SET search_path TO public AS 'SELECT arg'"#,
    ]);
}
