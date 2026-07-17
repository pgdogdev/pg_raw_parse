mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER ROLE
// Description: change a database role
// Syntax:
// ALTER ROLE role_specification [ WITH ] option [ ... ]
//
// where option can be:
//
//       SUPERUSER | NOSUPERUSER
//     | CREATEDB | NOCREATEDB
//     | CREATEROLE | NOCREATEROLE
//     | INHERIT | NOINHERIT
//     | LOGIN | NOLOGIN
//     | REPLICATION | NOREPLICATION
//     | BYPASSRLS | NOBYPASSRLS
//     | CONNECTION LIMIT connlimit
//     | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
//     | VALID UNTIL 'timestamp'
//
// ALTER ROLE name RENAME TO new_name
//
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter { TO | = } { value | DEFAULT }
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter FROM CURRENT
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] RESET configuration_parameter
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] RESET ALL
//
// where role_specification can be:
//
//     role_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
//
// URL: https://www.postgresql.org/docs/18/sql-alterrole.html

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
