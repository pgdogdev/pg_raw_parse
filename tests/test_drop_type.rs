mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_type_parses() {
    run_cases(&[
        r#"DROP TYPE my_type"#,
        r#"DROP TYPE IF EXISTS my_type, my_type_old CASCADE"#,
        r#"DROP TYPE IF EXISTS my_type RESTRICT"#,
    ]);
}
