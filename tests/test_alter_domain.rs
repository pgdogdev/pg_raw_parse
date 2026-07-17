mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER DOMAIN
// Description: change the definition of a domain
// Syntax:
// ALTER DOMAIN name
//     { SET DEFAULT expression | DROP DEFAULT }
// ALTER DOMAIN name
//     { SET | DROP } NOT NULL
// ALTER DOMAIN name
//     ADD domain_constraint [ NOT VALID ]
// ALTER DOMAIN name
//     DROP CONSTRAINT [ IF EXISTS ] constraint_name [ RESTRICT | CASCADE ]
// ALTER DOMAIN name
//      RENAME CONSTRAINT constraint_name TO new_constraint_name
// ALTER DOMAIN name
//     VALIDATE CONSTRAINT constraint_name
// ALTER DOMAIN name
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER DOMAIN name
//     RENAME TO new_name
// ALTER DOMAIN name
//     SET SCHEMA new_schema
//
// where domain_constraint is:
//
// [ CONSTRAINT constraint_name ]
// { NOT NULL | CHECK (expression) }
//
// URL: https://www.postgresql.org/docs/18/sql-alterdomain.html

#[test]
fn alter_domain_parses() {
    run_cases(&[
        r#"ALTER DOMAIN email_domain SET DEFAULT 'unknown@example.com'"#,
        r#"ALTER DOMAIN email_domain DROP DEFAULT"#,
        r#"ALTER DOMAIN email_domain SET NOT NULL"#,
        r#"ALTER DOMAIN email_domain DROP NOT NULL"#,
        r#"ALTER DOMAIN email_domain ADD CONSTRAINT email_check CHECK (VALUE LIKE '%@%') NOT VALID"#,
        r#"ALTER DOMAIN email_domain DROP CONSTRAINT IF EXISTS email_check CASCADE"#,
        r#"ALTER DOMAIN email_domain RENAME CONSTRAINT email_check TO email_check_new"#,
        r#"ALTER DOMAIN email_domain VALIDATE CONSTRAINT email_check"#,
        r#"ALTER DOMAIN email_domain OWNER TO CURRENT_USER"#,
        r#"ALTER DOMAIN email_domain RENAME TO email_domain_new"#,
        r#"ALTER DOMAIN email_domain SET SCHEMA public"#,
    ]);
}
