mod common;

use common::run_parse_debug_cases as run_cases;

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
