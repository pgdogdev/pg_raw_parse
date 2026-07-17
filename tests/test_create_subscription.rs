mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_subscription_parses() {
    run_cases(&[
        r#"CREATE SUBSCRIPTION sub CONNECTION 'host=localhost dbname=postgres' PUBLICATION pub"#,
        r#"CREATE SUBSCRIPTION sub CONNECTION 'host=localhost dbname=postgres' PUBLICATION pub1, pub2 WITH (copy_data = false, create_slot = false)"#,
    ]);
}
