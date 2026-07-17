mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn select_into_parses() {
    run_cases(&[
        r#"SELECT 1 AS id INTO new_table"#,
        r#"SELECT 1 AS id INTO TEMP TABLE new_table"#,
        r#"SELECT 1 AS id INTO UNLOGGED TABLE new_table FROM source_table WHERE id > 0"#,
    ]);
}
