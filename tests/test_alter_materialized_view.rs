mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER MATERIALIZED VIEW
// Description: change the definition of a materialized view
// Syntax:
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     action [, ... ]
// ALTER MATERIALIZED VIEW name
//     [ NO ] DEPENDS ON EXTENSION extension_name
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     RENAME [ COLUMN ] column_name TO new_column_name
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     RENAME TO new_name
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     SET SCHEMA new_schema
// ALTER MATERIALIZED VIEW ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
//     SET TABLESPACE new_tablespace [ NOWAIT ]
//
// where action is one of:
//
//     ALTER [ COLUMN ] column_name SET STATISTICS integer
//     ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
//     ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
//     ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
//     ALTER [ COLUMN ] column_name SET COMPRESSION compression_method
//     CLUSTER ON index_name
//     SET WITHOUT CLUSTER
//     SET ACCESS METHOD new_access_method
//     SET TABLESPACE new_tablespace
//     SET ( storage_parameter [= value] [, ... ] )
//     RESET ( storage_parameter [, ... ] )
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// URL: https://www.postgresql.org/docs/18/sql-altermaterializedview.html

#[test]
fn alter_materialized_view_parses() {
    run_cases(&[
        r#"ALTER MATERIALIZED VIEW IF EXISTS mv RENAME TO mv_new"#,
        r#"ALTER MATERIALIZED VIEW mv ALTER COLUMN col SET STATISTICS 100"#,
        r#"ALTER MATERIALIZED VIEW mv ALTER COLUMN col SET (n_distinct = 10)"#,
        r#"ALTER MATERIALIZED VIEW mv ALTER COLUMN col RESET (n_distinct)"#,
        r#"ALTER MATERIALIZED VIEW mv CLUSTER ON mv_idx"#,
        r#"ALTER MATERIALIZED VIEW mv SET WITHOUT CLUSTER"#,
        r#"ALTER MATERIALIZED VIEW mv SET ACCESS METHOD heap"#,
        r#"ALTER MATERIALIZED VIEW mv SET TABLESPACE fastspace"#,
        r#"ALTER MATERIALIZED VIEW mv SET (fillfactor = 80)"#,
        r#"ALTER MATERIALIZED VIEW mv RESET (fillfactor)"#,
        r#"ALTER MATERIALIZED VIEW mv OWNER TO CURRENT_USER"#,
        r#"ALTER MATERIALIZED VIEW mv SET SCHEMA public"#,
        r#"ALTER MATERIALIZED VIEW ALL IN TABLESPACE oldspace OWNED BY app_user SET TABLESPACE newspace NOWAIT"#,
    ]);
}
