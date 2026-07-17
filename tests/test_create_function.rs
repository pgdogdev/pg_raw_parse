mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_function_parses() {
    run_cases(&[
        r#"CREATE FUNCTION my_function() RETURNS integer LANGUAGE SQL AS 'SELECT 1'"#,
        r#"CREATE OR REPLACE FUNCTION my_function(arg integer DEFAULT 1) RETURNS TABLE (value integer) LANGUAGE SQL STABLE AS 'SELECT arg'"#,
        r#"CREATE FUNCTION my_function(IN arg integer, OUT value integer) RETURNS integer LANGUAGE SQL STRICT SECURITY DEFINER COST 5 ROWS 1 SET search_path TO public AS 'SELECT arg'"#,
    ]);
}
