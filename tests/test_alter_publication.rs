mod common;

use common::run_parse_debug_cases as run_cases;

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
