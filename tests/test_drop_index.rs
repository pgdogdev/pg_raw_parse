mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_index_parses() {
    run_cases(&[
        r#"DROP INDEX my_index"#,
        r#"DROP INDEX IF EXISTS my_index, my_index_old CASCADE"#,
        r#"DROP INDEX IF EXISTS my_index RESTRICT"#,
    ]);
}
