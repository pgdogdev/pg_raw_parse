mod common;

use common::run_parse_debug_cases as run_cases;

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
