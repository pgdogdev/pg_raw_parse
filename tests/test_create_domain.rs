mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_domain_parses() {
    run_cases(&[
        r#"CREATE DOMAIN email_domain AS text CHECK (VALUE LIKE '%@%')"#,
        r#"CREATE DOMAIN email_domain AS text COLLATE "C" DEFAULT 'x@example.com' NOT NULL CONSTRAINT email_check CHECK (VALUE LIKE '%@%')"#,
    ]);
}
