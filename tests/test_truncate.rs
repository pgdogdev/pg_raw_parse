mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn truncate_parses() {
    run_cases(&[
        r#"TRUNCATE my_table"#,
        r#"TRUNCATE TABLE ONLY my_table, other_table RESTART IDENTITY CASCADE"#,
        r#"TRUNCATE TABLE my_table CONTINUE IDENTITY RESTRICT"#,
    ]);
}
