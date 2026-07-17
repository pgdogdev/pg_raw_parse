mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER PUBLICATION
// Description: change the definition of a publication
// Syntax:
// ALTER PUBLICATION name ADD publication_object [, ...]
// ALTER PUBLICATION name SET publication_object [, ...]
// ALTER PUBLICATION name DROP publication_drop_object [, ...]
// ALTER PUBLICATION name SET ( publication_parameter [= value] [, ... ] )
// ALTER PUBLICATION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER PUBLICATION name RENAME TO new_name
//
// where publication_object is one of:
//
//     TABLE table_and_columns [, ... ]
//     TABLES IN SCHEMA { schema_name | CURRENT_SCHEMA } [, ... ]
//
// and publication_drop_object is one of:
//
//     TABLE [ ONLY ] table_name [ * ] [, ... ]
//     TABLES IN SCHEMA { schema_name | CURRENT_SCHEMA } [, ... ]
//
// and table_and_columns is:
//
//     [ ONLY ] table_name [ * ] [ ( column_name [, ... ] ) ] [ WHERE ( expression ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-alterpublication.html

#[test]
fn alter_publication_parses() {
    run_cases(&[
        r#"ALTER PUBLICATION pub ADD TABLE users, orders"#,
        r#"ALTER PUBLICATION pub SET TABLE users WHERE (id > 0)"#,
        r#"ALTER PUBLICATION pub DROP TABLE users"#,
        r#"ALTER PUBLICATION pub ADD TABLES IN SCHEMA public, sales"#,
        r#"ALTER PUBLICATION pub SET TABLES IN SCHEMA public"#,
        r#"ALTER PUBLICATION pub DROP TABLES IN SCHEMA sales"#,
        r#"ALTER PUBLICATION pub SET (publish = 'insert, update', publish_via_partition_root = true)"#,
        r#"ALTER PUBLICATION pub OWNER TO CURRENT_USER"#,
        r#"ALTER PUBLICATION pub RENAME TO pub_new"#,
    ]);
}
