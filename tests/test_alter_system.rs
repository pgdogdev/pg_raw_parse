mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_system_parses() {
    run_cases(&[
        r#"ALTER SYSTEM SET work_mem TO '64MB'"#,
        r#"ALTER SYSTEM SET work_mem = '64MB'"#,
        r#"ALTER SYSTEM RESET work_mem"#,
        r#"ALTER SYSTEM RESET ALL"#,
    ]);
}
