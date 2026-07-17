mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_view_parses() {
    run_cases(&[
        r#"DROP VIEW v"#,
        r#"DROP VIEW IF EXISTS v, v_old CASCADE"#,
        r#"DROP VIEW IF EXISTS v RESTRICT"#,
    ]);
}
