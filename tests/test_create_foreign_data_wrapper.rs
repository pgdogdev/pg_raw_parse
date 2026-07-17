mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_foreign_data_wrapper_parses() {
    run_cases(&[
        r#"CREATE FOREIGN DATA WRAPPER my_fdw HANDLER my_handler VALIDATOR my_validator OPTIONS (host 'localhost')"#,
        r#"CREATE FOREIGN DATA WRAPPER my_fdw NO HANDLER NO VALIDATOR"#,
    ]);
}
