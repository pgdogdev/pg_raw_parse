mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn security_label_parses() {
    run_cases(&["SECURITY LABEL ON TABLE my_table IS 'classified'"]);
}
