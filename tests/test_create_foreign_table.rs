mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_foreign_table_parses() {
    run_cases(&[
        r#"CREATE FOREIGN TABLE ft (id integer OPTIONS (column_name 'id'), body text COLLATE "C" NOT NULL) SERVER my_server OPTIONS (schema_name 'public', table_name 'remote')"#,
        r#"CREATE FOREIGN TABLE IF NOT EXISTS ft PARTITION OF parent FOR VALUES IN (1) SERVER my_server"#,
        r#"CREATE FOREIGN TABLE ft (LIKE source INCLUDING DEFAULTS, CHECK (id > 0), FOREIGN KEY (id) REFERENCES other(id)) SERVER my_server"#,
    ]);
}
