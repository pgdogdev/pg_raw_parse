mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_type_parses() {
    run_cases(&[
        r#"ALTER TYPE my_type OWNER TO CURRENT_USER"#,
        r#"ALTER TYPE my_type RENAME TO my_type_new"#,
        r#"ALTER TYPE my_type SET SCHEMA public"#,
        r#"ALTER TYPE my_type RENAME ATTRIBUTE old_attr TO new_attr CASCADE"#,
        r#"ALTER TYPE my_type ADD ATTRIBUTE new_attr integer COLLATE "C" RESTRICT"#,
        r#"ALTER TYPE my_type DROP ATTRIBUTE IF EXISTS old_attr CASCADE"#,
        r#"ALTER TYPE my_type ALTER ATTRIBUTE attr TYPE text COLLATE "C" CASCADE"#,
        r#"ALTER TYPE mood ADD VALUE 'happy' BEFORE 'sad'"#,
        r#"ALTER TYPE mood RENAME VALUE 'happy' TO 'joyful'"#,
        r#"ALTER TYPE my_type SET (RECEIVE = my_receive, SEND = my_send)"#,
        r#"ALTER TYPE my_type SET (STORAGE = extended)"#,
    ]);
}
