mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_domain_parses() {
    run_cases(&[
        r#"DROP DOMAIN email_domain"#,
        r#"DROP DOMAIN IF EXISTS email_domain, email_domain_old CASCADE"#,
        r#"DROP DOMAIN IF EXISTS email_domain RESTRICT"#,
    ]);
}
