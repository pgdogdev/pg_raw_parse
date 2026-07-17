mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     TRUNCATE
// Description: empty a table or set of tables
// Syntax:
// TRUNCATE [ TABLE ] [ ONLY ] name [ * ] [, ... ]
//     [ RESTART IDENTITY | CONTINUE IDENTITY ] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-truncate.html

#[test]
fn truncate_parses() {
    run_cases(&[
        r#"TRUNCATE my_table"#,
        r#"TRUNCATE TABLE ONLY my_table, other_table RESTART IDENTITY CASCADE"#,
        r#"TRUNCATE TABLE my_table CONTINUE IDENTITY RESTRICT"#,
    ]);
}
