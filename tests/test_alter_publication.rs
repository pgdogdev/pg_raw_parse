mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_publication_parses() {
    run_cases(&["ALTER PUBLICATION my_publication RENAME TO my_publication2"]);
}
