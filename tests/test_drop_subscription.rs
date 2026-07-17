mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP SUBSCRIPTION
// Description: remove a subscription
// Syntax:
// DROP SUBSCRIPTION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropsubscription.html

#[test]
fn drop_subscription_parses() {
    run_cases(&[
        r#"DROP SUBSCRIPTION sub"#,
        r#"DROP SUBSCRIPTION IF EXISTS sub CASCADE"#,
        r#"DROP SUBSCRIPTION IF EXISTS sub RESTRICT"#,
    ]);
}
