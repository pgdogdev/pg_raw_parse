mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_tablespace_parses() {
    run_cases(&[
        r#"DROP TABLESPACE fastspace"#,
        r#"DROP TABLESPACE IF EXISTS fastspace"#,
    ]);
}
