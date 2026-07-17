mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE DOMAIN
// Description: define a new domain
// Syntax:
// CREATE DOMAIN name [ AS ] data_type
//     [ COLLATE collation ]
//     [ DEFAULT expression ]
//     [ domain_constraint [ ... ] ]
//
// where domain_constraint is:
//
// [ CONSTRAINT constraint_name ]
// { NOT NULL | NULL | CHECK (expression) }
//
// URL: https://www.postgresql.org/docs/18/sql-createdomain.html

#[test]
fn create_domain_parses() {
    run_cases(&[
        r#"CREATE DOMAIN email_domain AS text CHECK (VALUE LIKE '%@%')"#,
        r#"CREATE DOMAIN email_domain AS text COLLATE "C" DEFAULT 'x@example.com' NOT NULL CONSTRAINT email_check CHECK (VALUE LIKE '%@%')"#,
    ]);
}
