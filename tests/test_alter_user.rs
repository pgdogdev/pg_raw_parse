mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER USER
// Description: change a database role
// Syntax:
// ALTER USER role_specification [ WITH ] option [ ... ]
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
// ALTER USER name RENAME TO new_name
//
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter { TO | = } { value | DEFAULT }
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter FROM CURRENT
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] RESET configuration_parameter
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] RESET ALL
//
// where role_specification can be:
//
//     role_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
//
// URL: https://www.postgresql.org/docs/18/sql-alteruser.html

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
