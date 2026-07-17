mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TRANSFORM
// Description: define a new transform
// Syntax:
// CREATE [ OR REPLACE ] TRANSFORM FOR type_name LANGUAGE lang_name (
//     FROM SQL WITH FUNCTION from_sql_function_name [ (argument_type [, ...]) ],
//     TO SQL WITH FUNCTION to_sql_function_name [ (argument_type [, ...]) ]
// );
//
// URL: https://www.postgresql.org/docs/18/sql-createtransform.html

#[test]
fn create_transform_parses() {
    run_cases(&[
        r#"CREATE TRANSFORM FOR my_type LANGUAGE plsample (FROM SQL WITH FUNCTION my_from_sql(my_type), TO SQL WITH FUNCTION my_to_sql(internal))"#,
        r#"CREATE OR REPLACE TRANSFORM FOR my_type LANGUAGE plsample (FROM SQL WITH FUNCTION my_from_sql(my_type))"#,
    ]);
}
