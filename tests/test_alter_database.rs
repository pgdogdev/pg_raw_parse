mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER DATABASE
// Description: change a database
// Syntax:
// ALTER DATABASE name [ [ WITH ] option [ ... ] ]
//
// where option can be:
//
//     ALLOW_CONNECTIONS allowconn
//     CONNECTION LIMIT connlimit
//     IS_TEMPLATE istemplate
//
// ALTER DATABASE name RENAME TO new_name
//
// ALTER DATABASE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// ALTER DATABASE name SET TABLESPACE new_tablespace
//
// ALTER DATABASE name REFRESH COLLATION VERSION
//
// ALTER DATABASE name SET configuration_parameter { TO | = } { value | DEFAULT }
// ALTER DATABASE name SET configuration_parameter FROM CURRENT
// ALTER DATABASE name RESET configuration_parameter
// ALTER DATABASE name RESET ALL
//
// URL: https://www.postgresql.org/docs/18/sql-alterdatabase.html

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
