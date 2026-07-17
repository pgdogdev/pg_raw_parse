mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_statistics_parses() {
    run_cases(&[
        r#"CREATE STATISTICS stats ON id, name FROM users"#,
        r#"CREATE STATISTICS IF NOT EXISTS stats (dependencies, ndistinct, mcv) ON id, lower(name) FROM users"#,
    ]);
}
