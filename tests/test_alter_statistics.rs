mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_statistics_parses() {
    run_cases(&[
        r#"ALTER STATISTICS stats SET STATISTICS 1000"#,
        r#"ALTER STATISTICS stats OWNER TO CURRENT_USER"#,
        r#"ALTER STATISTICS stats RENAME TO stats_new"#,
        r#"ALTER STATISTICS stats SET SCHEMA public"#,
    ]);
}
