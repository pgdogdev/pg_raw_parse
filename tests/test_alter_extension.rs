mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_extension_parses() {
    run_cases(&[
        r#"ALTER EXTENSION hstore UPDATE"#,
        r#"ALTER EXTENSION hstore UPDATE TO '1.8'"#,
        r#"ALTER EXTENSION hstore SET SCHEMA public"#,
        r#"ALTER EXTENSION hstore ADD TABLE my_table"#,
        r#"ALTER EXTENSION hstore DROP FUNCTION my_function(integer)"#,
    ]);
}
