mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_subscription_parses() {
    run_cases(&[
        "CREATE SUBSCRIPTION my_subscription CONNECTION 'host=localhost' PUBLICATION my_publication",
    ]);
}
