mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_role_parses() {
    run_cases(&[
        r#"ALTER ROLE app_user WITH LOGIN CREATEDB PASSWORD 'secret'"#,
        r#"ALTER ROLE app_user RENAME TO app_user_new"#,
        r#"ALTER ROLE app_user IN DATABASE mydb SET search_path TO public"#,
        r#"ALTER ROLE app_user SET work_mem = '64MB'"#,
        r#"ALTER ROLE app_user SET enable_seqscan FROM CURRENT"#,
        r#"ALTER ROLE app_user RESET work_mem"#,
        r#"ALTER ROLE app_user RESET ALL"#,
        r#"ALTER ROLE ALL IN DATABASE mydb SET statement_timeout TO '5s'"#,
    ]);
}
