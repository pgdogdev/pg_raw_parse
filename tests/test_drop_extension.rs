mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_extension_parses() {
    run_cases(&[
        r#"DROP EXTENSION hstore"#,
        r#"DROP EXTENSION IF EXISTS hstore, hstore_old CASCADE"#,
        r#"DROP EXTENSION IF EXISTS hstore RESTRICT"#,
    ]);
}
