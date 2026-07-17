mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_collation_parses() {
    run_cases(&[
        r#"CREATE COLLATION my_collation (provider = libc, locale = 'C')"#,
        r#"CREATE COLLATION IF NOT EXISTS my_collation FROM "C""#,
    ]);
}
