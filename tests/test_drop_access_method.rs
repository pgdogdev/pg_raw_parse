mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP ACCESS METHOD
// Description: remove an access method
// Syntax:
// DROP ACCESS METHOD [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-drop-access-method.html

#[test]
fn drop_access_method_parses() {
    run_cases(&[
        r#"DROP ACCESS METHOD my_am"#,
        r#"DROP ACCESS METHOD IF EXISTS my_am CASCADE"#,
    ]);
}
