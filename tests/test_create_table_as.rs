mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TABLE AS
// Description: define a new table from the results of a query
// Syntax:
// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name
//     [ (column_name [, ...] ) ]
//     [ USING method ]
//     [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
//     [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
//     [ TABLESPACE tablespace_name ]
//     AS query
//     [ WITH [ NO ] DATA ]
//
// URL: https://www.postgresql.org/docs/18/sql-createtableas.html

#[test]
fn create_table_as_parses() {
    run_cases(&[
        r#"CREATE TABLE new_table AS SELECT 1 AS id"#,
        r#"CREATE TEMP TABLE IF NOT EXISTS new_table (id, name) USING heap WITH (fillfactor = 80) ON COMMIT DROP TABLESPACE fastspace AS SELECT 1 AS id, 'x' AS name WITH NO DATA"#,
    ]);
}
