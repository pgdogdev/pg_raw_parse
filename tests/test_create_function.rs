mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE FUNCTION
// Description: define a new function
// Syntax:
// CREATE [ OR REPLACE ] FUNCTION
//     name ( [ [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ] [, ...] ] )
//     [ RETURNS rettype
//       | RETURNS TABLE ( column_name column_type [, ...] ) ]
//   { LANGUAGE lang_name
//     | TRANSFORM { FOR TYPE type_name } [, ... ]
//     | WINDOW
//     | { IMMUTABLE | STABLE | VOLATILE }
//     | [ NOT ] LEAKPROOF
//     | { CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT }
//     | { [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER }
//     | PARALLEL { UNSAFE | RESTRICTED | SAFE }
//     | COST execution_cost
//     | ROWS result_rows
//     | SUPPORT support_function
//     | SET configuration_parameter { TO value | = value | FROM CURRENT }
//     | AS 'definition'
//     | AS 'obj_file', 'link_symbol'
//     | sql_body
//   } ...
//
// URL: https://www.postgresql.org/docs/18/sql-createfunction.html

#[test]
fn create_function_parses() {
    run_cases(&[
        r#"CREATE FUNCTION my_function() RETURNS integer LANGUAGE SQL AS 'SELECT 1'"#,
        r#"CREATE OR REPLACE FUNCTION my_function(arg integer DEFAULT 1) RETURNS TABLE (value integer) LANGUAGE SQL STABLE AS 'SELECT arg'"#,
        r#"CREATE FUNCTION my_function(IN arg integer, OUT value integer) RETURNS integer LANGUAGE SQL STRICT SECURITY DEFINER COST 5 ROWS 1 SET search_path TO public AS 'SELECT arg'"#,
    ]);
}
