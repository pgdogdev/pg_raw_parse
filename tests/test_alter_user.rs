mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_user_parses() {
    run_cases(&[
        r#"ALTER USER app_user WITH LOGIN PASSWORD 'secret'"#,
        r#"ALTER USER app_user RENAME TO app_user_new"#,
        r#"ALTER USER app_user SET search_path TO public"#,
        r#"ALTER USER app_user IN DATABASE mydb SET work_mem TO '64MB'"#,
        r#"ALTER USER app_user RESET work_mem"#,
        r#"ALTER USER app_user RESET ALL"#,
    ]);
}
