mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER DEFAULT PRIVILEGES
// Description: define default access privileges
// Syntax:
// ALTER DEFAULT PRIVILEGES
//     [ FOR { ROLE | USER } target_role [, ...] ]
//     [ IN SCHEMA schema_name [, ...] ]
//     abbreviated_grant_or_revoke
//
// where abbreviated_grant_or_revoke is one of:
//
// GRANT { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON TABLES
//     TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
// GRANT { { USAGE | SELECT | UPDATE }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON SEQUENCES
//     TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
// GRANT { EXECUTE | ALL [ PRIVILEGES ] }
//     ON { FUNCTIONS | ROUTINES }
//     TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
// GRANT { USAGE | ALL [ PRIVILEGES ] }
//     ON TYPES
//     TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
// GRANT { { USAGE | CREATE }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON SCHEMAS
//     TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
// GRANT { { SELECT | UPDATE }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON LARGE OBJECTS
//     TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
// REVOKE [ GRANT OPTION FOR ]
//     { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON TABLES
//     FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//     [ CASCADE | RESTRICT ]
//
// REVOKE [ GRANT OPTION FOR ]
//     { { USAGE | SELECT | UPDATE }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON SEQUENCES
//     FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//     [ CASCADE | RESTRICT ]
//
// REVOKE [ GRANT OPTION FOR ]
//     { EXECUTE | ALL [ PRIVILEGES ] }
//     ON { FUNCTIONS | ROUTINES }
//     FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//     [ CASCADE | RESTRICT ]
//
// REVOKE [ GRANT OPTION FOR ]
//     { USAGE | ALL [ PRIVILEGES ] }
//     ON TYPES
//     FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//     [ CASCADE | RESTRICT ]
//
// REVOKE [ GRANT OPTION FOR ]
//     { { USAGE | CREATE }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON SCHEMAS
//     FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//     [ CASCADE | RESTRICT ]
//
// REVOKE [ GRANT OPTION FOR ]
//     { { SELECT | UPDATE }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON LARGE OBJECTS
//     FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-alterdefaultprivileges.html

#[test]
fn alter_default_privileges_parses() {
    run_cases(&[
        r#"ALTER DEFAULT PRIVILEGES GRANT SELECT ON TABLES TO app_user"#,
        r#"ALTER DEFAULT PRIVILEGES FOR ROLE owner_role IN SCHEMA public GRANT USAGE ON TYPES TO app_user WITH GRANT OPTION"#,
        r#"ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE GRANT OPTION FOR SELECT ON TABLES FROM app_user CASCADE"#,
        r#"ALTER DEFAULT PRIVILEGES REVOKE EXECUTE ON ROUTINES FROM PUBLIC RESTRICT"#,
    ]);
}
