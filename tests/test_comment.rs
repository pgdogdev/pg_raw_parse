mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn comment_parses() {
    run_cases(&["COMMENT ON TABLE my_table IS 'comment'"]);
}
