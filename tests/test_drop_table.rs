mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_table_parses() {
    run_cases(&[
        r#"DROP TABLE my_table"#,
        r#"DROP TABLE IF EXISTS my_table, my_table_old CASCADE"#,
        r#"DROP TABLE IF EXISTS my_table RESTRICT"#,
    ]);
}
