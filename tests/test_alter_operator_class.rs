mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_operator_class_parses() {
    run_cases(&[
        r#"ALTER OPERATOR CLASS my_opclass USING btree RENAME TO my_opclass_new"#,
        r#"ALTER OPERATOR CLASS my_opclass USING btree OWNER TO CURRENT_USER"#,
        r#"ALTER OPERATOR CLASS my_opclass USING btree SET SCHEMA public"#,
    ]);
}
