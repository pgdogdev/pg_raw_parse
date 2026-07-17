mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_operator_family_parses() {
    run_cases(&[
        r#"ALTER OPERATOR FAMILY my_family USING btree ADD OPERATOR 1 =(integer, integer)"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree DROP OPERATOR 1 (integer, integer)"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree RENAME TO my_family_new"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree OWNER TO CURRENT_USER"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree SET SCHEMA public"#,
    ]);
}
