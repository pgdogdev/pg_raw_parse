mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_index_parses() {
    run_cases(&[
        r#"CREATE INDEX my_index ON my_table (id)"#,
        r#"CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS my_index ON ONLY my_table USING btree (id COLLATE "C" int4_ops ASC NULLS LAST, (lower(name)) DESC) INCLUDE (email) NULLS NOT DISTINCT WITH (fillfactor = 80) TABLESPACE fastspace WHERE id > 0"#,
    ]);
}
