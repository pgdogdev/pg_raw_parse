mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_publication_parses() {
    run_cases(&["DROP PUBLICATION IF EXISTS my_publication"]);
}
