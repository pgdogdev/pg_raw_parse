mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP DOMAIN
// Description: remove a domain
// Syntax:
// DROP DOMAIN [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropdomain.html

#[test]
fn drop_domain_parses() {
    run_cases(&[
        r#"DROP DOMAIN email_domain"#,
        r#"DROP DOMAIN IF EXISTS email_domain, email_domain_old CASCADE"#,
        r#"DROP DOMAIN IF EXISTS email_domain RESTRICT"#,
    ]);
}
