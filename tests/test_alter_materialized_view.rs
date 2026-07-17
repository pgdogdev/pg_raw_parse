mod common;

use common::run_parse_debug_cases as run_cases;

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
