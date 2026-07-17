mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE SUBSCRIPTION
// Description: define a new subscription
// Syntax:
// CREATE SUBSCRIPTION subscription_name
//     CONNECTION 'conninfo'
//     PUBLICATION publication_name [, ...]
//     [ WITH ( subscription_parameter [= value] [, ... ] ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createsubscription.html

#[test]
fn create_subscription_parses() {
    run_cases(&[
        r#"CREATE SUBSCRIPTION sub CONNECTION 'host=localhost dbname=postgres' PUBLICATION pub"#,
        r#"CREATE SUBSCRIPTION sub CONNECTION 'host=localhost dbname=postgres' PUBLICATION pub1, pub2 WITH (copy_data = false, create_slot = false)"#,
    ]);
}
