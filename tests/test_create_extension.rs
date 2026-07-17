mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_extension_parses() {
    run_cases(&[
        r#"CREATE EXTENSION hstore"#,
        r#"CREATE EXTENSION IF NOT EXISTS hstore WITH SCHEMA public VERSION '1.8' CASCADE"#,
    ]);
}
