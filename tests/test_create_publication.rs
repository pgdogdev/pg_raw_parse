mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_publication_parses() {
    run_cases(&[
        r#"CREATE PUBLICATION pub FOR ALL TABLES"#,
        r#"CREATE PUBLICATION pub FOR TABLE users, ONLY orders (id, total) WHERE (total > 0), TABLES IN SCHEMA public WITH (publish = 'insert, update', publish_via_partition_root = true)"#,
        r#"CREATE PUBLICATION pub WITH (publish = 'truncate')"#,
    ]);
}
