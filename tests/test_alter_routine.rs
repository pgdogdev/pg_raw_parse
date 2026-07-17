mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER ROUTINE
// Description: change the definition of a routine
// Syntax:
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     action [ ... ] [ RESTRICT ]
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     RENAME TO new_name
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     SET SCHEMA new_schema
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     [ NO ] DEPENDS ON EXTENSION extension_name
//
// where action is one of:
//
//     IMMUTABLE | STABLE | VOLATILE
//     [ NOT ] LEAKPROOF
//     [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     PARALLEL { UNSAFE | RESTRICTED | SAFE }
//     COST execution_cost
//     ROWS result_rows
//     SET configuration_parameter { TO | = } { value | DEFAULT }
//     SET configuration_parameter FROM CURRENT
//     RESET configuration_parameter
//     RESET ALL
//
// URL: https://www.postgresql.org/docs/18/sql-alterroutine.html

#[test]
fn alter_routine_parses() {
    run_cases(&[
        r#"ALTER ROUTINE my_function(integer) IMMUTABLE PARALLEL SAFE COST 10"#,
        r#"ALTER ROUTINE my_function(integer) SET search_path TO public"#,
        r#"ALTER ROUTINE my_function(integer) RESET ALL"#,
        r#"ALTER ROUTINE my_function(integer) OWNER TO CURRENT_USER"#,
        r#"ALTER ROUTINE my_function(integer) RENAME TO my_function_new"#,
        r#"ALTER ROUTINE my_function(integer) SET SCHEMA public"#,
        r#"ALTER ROUTINE my_function(integer) DEPENDS ON EXTENSION hstore"#,
        r#"ALTER ROUTINE my_function(integer) NO DEPENDS ON EXTENSION hstore"#,
    ]);
}
