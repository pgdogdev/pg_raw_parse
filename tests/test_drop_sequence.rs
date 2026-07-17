mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_sequence_parses() {
    run_cases(&[
        r#"DROP SEQUENCE seq"#,
        r#"DROP SEQUENCE IF EXISTS seq, seq_old CASCADE"#,
        r#"DROP SEQUENCE IF EXISTS seq RESTRICT"#,
    ]);
}
