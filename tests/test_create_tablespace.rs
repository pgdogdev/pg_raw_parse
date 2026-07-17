mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_tablespace_parses() {
    run_cases(&[
        r#"CREATE TABLESPACE fastspace LOCATION '/tmp/fastspace'"#,
        r#"CREATE TABLESPACE fastspace OWNER app_user LOCATION '/tmp/fastspace' WITH (random_page_cost = 1.1)"#,
    ]);
}
