mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE USER
// Description: define a new database role
// Syntax:
// CREATE USER name [ [ WITH ] option [ ... ] ]
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
//     | IN ROLE role_name [, ...]
//     | IN GROUP role_name [, ...]
//     | ROLE role_name [, ...]
//     | ADMIN role_name [, ...]
//     | USER role_name [, ...]
//     | SYSID uid
//
// URL: https://www.postgresql.org/docs/18/sql-createuser.html

#[test]
fn create_user_parses() {
    run_cases(&[
        r#"CREATE USER app_user"#,
        r#"CREATE USER app_user WITH SUPERUSER CREATEDB CREATEROLE INHERIT LOGIN REPLICATION BYPASSRLS CONNECTION LIMIT 5 PASSWORD 'secret' VALID UNTIL 'infinity' IN ROLE parent_role ROLE child_role ADMIN admin_role USER legacy_user SYSID 10"#,
    ]);
}
