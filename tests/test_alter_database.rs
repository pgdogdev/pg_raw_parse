mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_database_parses() {
    run_cases(&[
        r#"ALTER DATABASE mydb WITH ALLOW_CONNECTIONS false"#,
        r#"ALTER DATABASE mydb RENAME TO mydb2"#,
        r#"ALTER DATABASE mydb OWNER TO CURRENT_USER"#,
        r#"ALTER DATABASE mydb SET TABLESPACE fastspace"#,
        r#"ALTER DATABASE mydb SET search_path TO public, extensions"#,
        r#"ALTER DATABASE mydb SET work_mem = '64MB'"#,
        r#"ALTER DATABASE mydb SET enable_seqscan FROM CURRENT"#,
        r#"ALTER DATABASE mydb RESET work_mem"#,
        r#"ALTER DATABASE mydb RESET ALL"#,
    ]);
}
