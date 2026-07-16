mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_domain_parses() {
    run_cases(&["ALTER DOMAIN my_domain SET NOT NULL"]);
}
