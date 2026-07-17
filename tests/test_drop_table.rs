mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TABLE
// Description: remove a table
// Syntax:
// DROP TABLE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptable.html

#[test]
fn drop_table_parses() {
    run_cases(&[
        r#"DROP TABLE my_table"#,
        r#"DROP TABLE IF EXISTS my_table, my_table_old CASCADE"#,
        r#"DROP TABLE IF EXISTS my_table RESTRICT"#,
    ]);
}
