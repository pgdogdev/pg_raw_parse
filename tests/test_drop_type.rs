mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TYPE
// Description: remove a data type
// Syntax:
// DROP TYPE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptype.html

#[test]
fn drop_type_parses() {
    run_cases(&[
        r#"DROP TYPE my_type"#,
        r#"DROP TYPE IF EXISTS my_type, my_type_old CASCADE"#,
        r#"DROP TYPE IF EXISTS my_type RESTRICT"#,
    ]);
}
