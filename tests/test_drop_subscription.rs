mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_subscription_parses() {
    run_cases(&["DROP SUBSCRIPTION IF EXISTS my_subscription"]);
}
