mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP STATISTICS
// Description: remove extended statistics
// Syntax:
// DROP STATISTICS [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropstatistics.html

#[test]
fn drop_statistics_parses() {
    run_cases(&[
        r#"DROP STATISTICS stats"#,
        r#"DROP STATISTICS IF EXISTS stats, stats_old CASCADE"#,
        r#"DROP STATISTICS IF EXISTS stats RESTRICT"#,
    ]);
}
