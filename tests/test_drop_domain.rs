mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_domain_parses() {
    run_cases(&["DROP DOMAIN IF EXISTS my_domain"]);
}
