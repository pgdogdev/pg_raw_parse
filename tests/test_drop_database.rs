mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_database_parses() {
    run_cases(&[
        r#"DROP DATABASE mydb"#,
        r#"DROP DATABASE IF EXISTS mydb WITH (FORCE)"#,
    ]);
}
