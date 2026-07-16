mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_transform_parses() {
    run_cases(&[
        "CREATE TRANSFORM FOR integer LANGUAGE plpython3u (FROM SQL WITH FUNCTION from_sql(integer), TO SQL WITH FUNCTION to_sql(integer))",
    ]);
}
