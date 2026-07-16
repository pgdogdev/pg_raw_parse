mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_domain_parses() {
    run_cases(&["CREATE DOMAIN my_domain AS integer CHECK (VALUE > 0)"]);
}
