mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER FUNCTION
// Description: change the definition of a function
// Syntax:
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     action [ ... ] [ RESTRICT ]
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     RENAME TO new_name
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     SET SCHEMA new_schema
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     [ NO ] DEPENDS ON EXTENSION extension_name
//
// where action is one of:
//
//     CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT
//     IMMUTABLE | STABLE | VOLATILE
//     [ NOT ] LEAKPROOF
//     [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     PARALLEL { UNSAFE | RESTRICTED | SAFE }
//     COST execution_cost
//     ROWS result_rows
//     SUPPORT support_function
//     SET configuration_parameter { TO | = } { value | DEFAULT }
//     SET configuration_parameter FROM CURRENT
//     RESET configuration_parameter
//     RESET ALL
//
// URL: https://www.postgresql.org/docs/18/sql-alterfunction.html

#[test]
fn alter_function_parses() {
    run_cases(&[
        r#"ALTER FUNCTION my_function(integer) CALLED ON NULL INPUT"#,
        r#"ALTER FUNCTION my_function(integer) RETURNS NULL ON NULL INPUT"#,
        r#"ALTER FUNCTION my_function(integer) STRICT"#,
        r#"ALTER FUNCTION my_function(integer) IMMUTABLE PARALLEL SAFE COST 5 ROWS 10 SET search_path TO public"#,
        r#"ALTER FUNCTION my_function(integer) RESET ALL"#,
        r#"ALTER FUNCTION my_function(integer) OWNER TO CURRENT_USER"#,
        r#"ALTER FUNCTION my_function(integer) RENAME TO my_function_new"#,
        r#"ALTER FUNCTION my_function(integer) SET SCHEMA public"#,
        r#"ALTER FUNCTION my_function(integer) DEPENDS ON EXTENSION hstore"#,
        r#"ALTER FUNCTION my_function(integer) NO DEPENDS ON EXTENSION hstore"#,
    ]);
}
