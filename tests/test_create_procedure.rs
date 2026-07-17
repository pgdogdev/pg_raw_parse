mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE PROCEDURE
// Description: define a new procedure
// Syntax:
// CREATE [ OR REPLACE ] PROCEDURE
//     name ( [ [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ] [, ...] ] )
//   { LANGUAGE lang_name
//     | TRANSFORM { FOR TYPE type_name } [, ... ]
//     | [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     | SET configuration_parameter { TO value | = value | FROM CURRENT }
//     | AS 'definition'
//     | AS 'obj_file', 'link_symbol'
//     | sql_body
//   } ...
//
// URL: https://www.postgresql.org/docs/18/sql-createprocedure.html

#[test]
fn create_procedure_parses() {
    run_cases(&[
        r#"CREATE PROCEDURE my_proc() LANGUAGE SQL AS 'SELECT 1'"#,
        r#"CREATE OR REPLACE PROCEDURE my_proc(IN arg integer DEFAULT 1, OUT value integer) LANGUAGE SQL SECURITY DEFINER SET search_path TO public AS 'SELECT arg'"#,
    ]);
}
