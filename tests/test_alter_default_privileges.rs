mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_default_privileges_parses() {
    run_cases(&["ALTER DEFAULT PRIVILEGES GRANT SELECT ON TABLES TO public"]);
}
