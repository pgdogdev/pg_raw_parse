mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_statistics_parses() {
    run_cases(&[
        r#"DROP STATISTICS stats"#,
        r#"DROP STATISTICS IF EXISTS stats, stats_old CASCADE"#,
        r#"DROP STATISTICS IF EXISTS stats RESTRICT"#,
    ]);
}
