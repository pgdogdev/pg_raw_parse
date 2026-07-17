mod common;

use common::run_parse_debug_cases as run_cases;

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
