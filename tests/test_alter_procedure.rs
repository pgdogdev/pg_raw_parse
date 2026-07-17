mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER PROCEDURE
// Description: change the definition of a procedure
// Syntax:
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     action [ ... ] [ RESTRICT ]
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     RENAME TO new_name
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     SET SCHEMA new_schema
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     [ NO ] DEPENDS ON EXTENSION extension_name
//
// where action is one of:
//
//     [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     SET configuration_parameter { TO | = } { value | DEFAULT }
//     SET configuration_parameter FROM CURRENT
//     RESET configuration_parameter
//     RESET ALL
//
// URL: https://www.postgresql.org/docs/18/sql-alterprocedure.html

#[test]
fn alter_procedure_parses() {
    run_cases(&[
        r#"ALTER PROCEDURE my_proc(integer) SECURITY DEFINER"#,
        r#"ALTER PROCEDURE my_proc(integer) SECURITY INVOKER"#,
        r#"ALTER PROCEDURE my_proc(integer) SET search_path TO public"#,
        r#"ALTER PROCEDURE my_proc(integer) RESET ALL"#,
        r#"ALTER PROCEDURE my_proc(integer) OWNER TO CURRENT_USER"#,
        r#"ALTER PROCEDURE my_proc(integer) RENAME TO my_proc_new"#,
        r#"ALTER PROCEDURE my_proc(integer) SET SCHEMA public"#,
        r#"ALTER PROCEDURE my_proc(integer) DEPENDS ON EXTENSION hstore"#,
        r#"ALTER PROCEDURE my_proc(integer) NO DEPENDS ON EXTENSION hstore"#,
    ]);
}
