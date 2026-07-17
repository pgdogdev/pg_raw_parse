mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_foreign_table_parses() {
    run_cases(&[
        r#"DROP FOREIGN TABLE ft"#,
        r#"DROP FOREIGN TABLE IF EXISTS ft, ft_old CASCADE"#,
        r#"DROP FOREIGN TABLE IF EXISTS ft RESTRICT"#,
    ]);
}
