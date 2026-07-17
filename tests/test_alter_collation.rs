mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_collation_parses() {
    run_cases(&[
        r#"ALTER COLLATION my_collation REFRESH VERSION"#,
        r#"ALTER COLLATION my_collation RENAME TO my_new_collation"#,
        r#"ALTER COLLATION my_collation OWNER TO CURRENT_USER"#,
        r#"ALTER COLLATION my_collation SET SCHEMA public"#,
    ]);
}
