mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER OPERATOR CLASS
// Description: change the definition of an operator class
// Syntax:
// ALTER OPERATOR CLASS name USING index_method
//     RENAME TO new_name
//
// ALTER OPERATOR CLASS name USING index_method
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// ALTER OPERATOR CLASS name USING index_method
//     SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-alteropclass.html

#[test]
fn alter_operator_class_parses() {
    run_cases(&[
        r#"ALTER OPERATOR CLASS my_opclass USING btree RENAME TO my_opclass_new"#,
        r#"ALTER OPERATOR CLASS my_opclass USING btree OWNER TO CURRENT_USER"#,
        r#"ALTER OPERATOR CLASS my_opclass USING btree SET SCHEMA public"#,
    ]);
}
