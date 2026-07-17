mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_default_privileges_parses() {
    run_cases(&[
        r#"ALTER DEFAULT PRIVILEGES GRANT SELECT ON TABLES TO app_user"#,
        r#"ALTER DEFAULT PRIVILEGES FOR ROLE owner_role IN SCHEMA public GRANT USAGE ON TYPES TO app_user WITH GRANT OPTION"#,
        r#"ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE GRANT OPTION FOR SELECT ON TABLES FROM app_user CASCADE"#,
        r#"ALTER DEFAULT PRIVILEGES REVOKE EXECUTE ON ROUTINES FROM PUBLIC RESTRICT"#,
    ]);
}
