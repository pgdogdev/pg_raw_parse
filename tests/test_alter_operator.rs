mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_operator_parses() {
    run_cases(&[
        r#"ALTER OPERATOR +(integer, integer) OWNER TO CURRENT_USER"#,
        r#"ALTER OPERATOR +(integer, integer) SET SCHEMA public"#,
        r#"ALTER OPERATOR +(integer, integer) SET (RESTRICT = my_restrict, JOIN = my_join)"#,
    ]);
}
