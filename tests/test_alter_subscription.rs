mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_subscription_parses() {
    run_cases(&["ALTER SUBSCRIPTION my_subscription ENABLE"]);
}
