mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_cast_parses() {
    run_cases(&[
        r#"DROP CAST (integer AS bigint)"#,
        r#"DROP CAST IF EXISTS (integer AS bigint) CASCADE"#,
    ]);
}
