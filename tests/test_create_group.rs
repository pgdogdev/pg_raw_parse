mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_group_parses() {
    run_cases(&[
        r#"CREATE GROUP my_group"#,
        r#"CREATE GROUP my_group WITH USER alice, bob"#,
    ]);
}
