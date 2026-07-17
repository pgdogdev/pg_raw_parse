mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE GROUP
// Description: define a new database role
// Syntax:
// CREATE GROUP name [ [ WITH ] option [ ... ] ]
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
// URL: https://www.postgresql.org/docs/18/sql-creategroup.html

#[test]
fn create_group_parses() {
    run_cases(&[
        r#"CREATE GROUP my_group"#,
        r#"CREATE GROUP my_group WITH USER alice, bob"#,
    ]);
}
