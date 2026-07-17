mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn refresh_materialized_view_parses() {
    run_cases(&[
        r#"REFRESH MATERIALIZED VIEW mv"#,
        r#"REFRESH MATERIALIZED VIEW CONCURRENTLY mv WITH NO DATA"#,
        r#"REFRESH MATERIALIZED VIEW mv WITH DATA"#,
    ]);
}
