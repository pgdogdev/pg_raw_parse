mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE PUBLICATION
// Description: define a new publication
// Syntax:
// CREATE PUBLICATION name
//     [ FOR ALL TABLES
//       | FOR publication_object [, ... ] ]
//     [ WITH ( publication_parameter [= value] [, ... ] ) ]
//
// where publication_object is one of:
//
//     TABLE table_and_columns [, ... ]
//     TABLES IN SCHEMA { schema_name | CURRENT_SCHEMA } [, ... ]
//
// and table_and_columns is:
//
//     [ ONLY ] table_name [ * ] [ ( column_name [, ... ] ) ] [ WHERE ( expression ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createpublication.html

#[test]
fn create_publication_parses() {
    run_cases(&[
        r#"CREATE PUBLICATION pub FOR ALL TABLES"#,
        r#"CREATE PUBLICATION pub FOR TABLE users, ONLY orders (id, total) WHERE (total > 0), TABLES IN SCHEMA public WITH (publish = 'insert, update', publish_via_partition_root = true)"#,
        r#"CREATE PUBLICATION pub WITH (publish = 'truncate')"#,
    ]);
}
