mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TYPE
// Description: change the definition of a type
// Syntax:
// ALTER TYPE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TYPE name RENAME TO new_name
// ALTER TYPE name SET SCHEMA new_schema
// ALTER TYPE name RENAME ATTRIBUTE attribute_name TO new_attribute_name [ CASCADE | RESTRICT ]
// ALTER TYPE name action [, ... ]
// ALTER TYPE name ADD VALUE [ IF NOT EXISTS ] new_enum_value [ { BEFORE | AFTER } neighbor_enum_value ]
// ALTER TYPE name RENAME VALUE existing_enum_value TO new_enum_value
// ALTER TYPE name SET ( property = value [, ... ] )
//
// where action is one of:
//
//     ADD ATTRIBUTE attribute_name data_type [ COLLATE collation ] [ CASCADE | RESTRICT ]
//     DROP ATTRIBUTE [ IF EXISTS ] attribute_name [ CASCADE | RESTRICT ]
//     ALTER ATTRIBUTE attribute_name [ SET DATA ] TYPE data_type [ COLLATE collation ] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-altertype.html

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
