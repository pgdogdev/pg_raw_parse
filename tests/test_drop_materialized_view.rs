mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_materialized_view_parses() {
    run_cases(&[
        r#"DROP MATERIALIZED VIEW mv"#,
        r#"DROP MATERIALIZED VIEW IF EXISTS mv, mv_old CASCADE"#,
        r#"DROP MATERIALIZED VIEW IF EXISTS mv RESTRICT"#,
    ]);
}
