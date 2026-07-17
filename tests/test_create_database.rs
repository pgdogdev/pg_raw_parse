mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_database_parses() {
    run_cases(&[
        r#"CREATE DATABASE mydb"#,
        r#"CREATE DATABASE mydb WITH OWNER = app_user TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'C' TABLESPACE = pg_default ALLOW_CONNECTIONS = true CONNECTION LIMIT = 10 IS_TEMPLATE = false"#,
        r#"CREATE DATABASE mydb STRATEGY = WAL_LOG"#,
    ]);
}
