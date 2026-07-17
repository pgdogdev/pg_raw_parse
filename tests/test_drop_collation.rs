mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_collation_parses() {
    run_cases(&[
        r#"DROP COLLATION my_collation"#,
        r#"DROP COLLATION IF EXISTS my_collation, my_collation_old CASCADE"#,
        r#"DROP COLLATION IF EXISTS my_collation RESTRICT"#,
    ]);
}
