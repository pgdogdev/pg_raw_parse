mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_user_parses() {
    run_cases(&[
        r#"CREATE USER app_user"#,
        r#"CREATE USER app_user WITH SUPERUSER CREATEDB CREATEROLE INHERIT LOGIN REPLICATION BYPASSRLS CONNECTION LIMIT 5 PASSWORD 'secret' VALID UNTIL 'infinity' IN ROLE parent_role ROLE child_role ADMIN admin_role USER legacy_user SYSID 10"#,
    ]);
}
