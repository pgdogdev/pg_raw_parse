mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE ROLE
// Description: define a new database role
// Syntax:
// CREATE ROLE name [ [ WITH ] option [ ... ] ]
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
//     | ROLE role_name [, ...]
//     | ADMIN role_name [, ...]
//     | SYSID uid
//
// URL: https://www.postgresql.org/docs/18/sql-createrole.html

#[test]
fn create_role_parses() {
    run_cases(&[
        r#"CREATE ROLE app_user"#,
        r#"CREATE ROLE app_user WITH SUPERUSER CREATEDB CREATEROLE INHERIT LOGIN REPLICATION BYPASSRLS CONNECTION LIMIT 5 PASSWORD 'secret' VALID UNTIL 'infinity' IN ROLE parent_role ROLE child_role ADMIN admin_role USER legacy_user SYSID 10"#,
    ]);
}
