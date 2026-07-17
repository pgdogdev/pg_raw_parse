mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_access_method_parses() {
    run_cases(&[
        r#"CREATE ACCESS METHOD my_am TYPE INDEX HANDLER my_handler"#,
        r#"CREATE ACCESS METHOD my_table_am TYPE TABLE HANDLER my_table_handler"#,
    ]);
}
