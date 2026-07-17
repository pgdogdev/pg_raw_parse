mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     REINDEX
// Description: rebuild indexes
// Syntax:
// REINDEX [ ( option [, ...] ) ] { INDEX | TABLE | SCHEMA } [ CONCURRENTLY ] name
// REINDEX [ ( option [, ...] ) ] { DATABASE | SYSTEM } [ CONCURRENTLY ] [ name ]
//
// where option can be one of:
//
//     CONCURRENTLY [ boolean ]
//     TABLESPACE new_tablespace
//     VERBOSE [ boolean ]
//
// URL: https://www.postgresql.org/docs/18/sql-reindex.html

#[test]
fn reindex_parses() {
    run_cases(&[
        r#"REINDEX INDEX my_index"#,
        r#"REINDEX TABLE CONCURRENTLY my_table"#,
        r#"REINDEX SCHEMA public"#,
        r#"REINDEX DATABASE mydb"#,
        r#"REINDEX SYSTEM mydb"#,
        r#"REINDEX (VERBOSE true, TABLESPACE fastspace) INDEX CONCURRENTLY my_index"#,
    ]);
}
