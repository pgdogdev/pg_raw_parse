mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_group_parses() {
    run_cases(&[
        r#"DROP GROUP my_group"#,
        r#"DROP GROUP IF EXISTS my_group, my_group_old"#,
    ]);
}
