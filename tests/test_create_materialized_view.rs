mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE MATERIALIZED VIEW
// Description: define a new materialized view
// Syntax:
// CREATE MATERIALIZED VIEW [ IF NOT EXISTS ] table_name
//     [ (column_name [, ...] ) ]
//     [ USING method ]
//     [ WITH ( storage_parameter [= value] [, ... ] ) ]
//     [ TABLESPACE tablespace_name ]
//     AS query
//     [ WITH [ NO ] DATA ]
//
// URL: https://www.postgresql.org/docs/18/sql-creatematerializedview.html

#[test]
fn create_materialized_view_parses() {
    run_cases(&[
        r#"CREATE MATERIALIZED VIEW mv AS SELECT 1 AS col"#,
        r#"CREATE MATERIALIZED VIEW IF NOT EXISTS mv (col) USING heap WITH (fillfactor = 80) TABLESPACE fastspace AS SELECT 1 WITH NO DATA"#,
    ]);
}
