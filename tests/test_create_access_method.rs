mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE ACCESS METHOD
// Description: define a new access method
// Syntax:
// CREATE ACCESS METHOD name
//     TYPE access_method_type
//     HANDLER handler_function
//
// URL: https://www.postgresql.org/docs/18/sql-create-access-method.html

#[test]
fn create_access_method_parses() {
    run_cases(&[
        r#"CREATE ACCESS METHOD my_am TYPE INDEX HANDLER my_handler"#,
        r#"CREATE ACCESS METHOD my_table_am TYPE TABLE HANDLER my_table_handler"#,
    ]);
}
