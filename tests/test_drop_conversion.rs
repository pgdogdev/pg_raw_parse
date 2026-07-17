mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP CONVERSION
// Description: remove a conversion
// Syntax:
// DROP CONVERSION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropconversion.html

#[test]
fn drop_conversion_parses() {
    run_cases(&[
        r#"DROP CONVERSION my_conversion"#,
        r#"DROP CONVERSION IF EXISTS my_conversion, my_conversion_old CASCADE"#,
        r#"DROP CONVERSION IF EXISTS my_conversion RESTRICT"#,
    ]);
}
