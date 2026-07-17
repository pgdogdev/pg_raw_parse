mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_transform_parses() {
    run_cases(&[
        r#"CREATE TRANSFORM FOR my_type LANGUAGE plsample (FROM SQL WITH FUNCTION my_from_sql(my_type), TO SQL WITH FUNCTION my_to_sql(internal))"#,
        r#"CREATE OR REPLACE TRANSFORM FOR my_type LANGUAGE plsample (FROM SQL WITH FUNCTION my_from_sql(my_type))"#,
    ]);
}
