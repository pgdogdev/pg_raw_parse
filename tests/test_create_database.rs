mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE DATABASE
// Description: create a new database
// Syntax:
// CREATE DATABASE name
//     [ WITH ] [ OWNER [=] user_name ]
//            [ TEMPLATE [=] template ]
//            [ ENCODING [=] encoding ]
//            [ STRATEGY [=] strategy ]
//            [ LOCALE [=] locale ]
//            [ LC_COLLATE [=] lc_collate ]
//            [ LC_CTYPE [=] lc_ctype ]
//            [ BUILTIN_LOCALE [=] builtin_locale ]
//            [ ICU_LOCALE [=] icu_locale ]
//            [ ICU_RULES [=] icu_rules ]
//            [ LOCALE_PROVIDER [=] locale_provider ]
//            [ COLLATION_VERSION = collation_version ]
//            [ TABLESPACE [=] tablespace_name ]
//            [ ALLOW_CONNECTIONS [=] allowconn ]
//            [ CONNECTION LIMIT [=] connlimit ]
//            [ IS_TEMPLATE [=] istemplate ]
//            [ OID [=] oid ]
//
// URL: https://www.postgresql.org/docs/18/sql-createdatabase.html

#[test]
fn create_database_parses() {
    run_cases(&[
        r#"CREATE DATABASE mydb"#,
        r#"CREATE DATABASE mydb WITH OWNER = app_user TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'C' TABLESPACE = pg_default ALLOW_CONNECTIONS = true CONNECTION LIMIT = 10 IS_TEMPLATE = false"#,
        r#"CREATE DATABASE mydb STRATEGY = WAL_LOG"#,
    ]);
}
