mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP DATABASE
// Description: remove a database
// Syntax:
// DROP DATABASE [ IF EXISTS ] name [ [ WITH ] ( option [, ...] ) ]
//
// where option can be:
//
//     FORCE
//
// URL: https://www.postgresql.org/docs/18/sql-dropdatabase.html

#[test]
fn drop_database_parses() {
    run_cases(&[
        r#"DROP DATABASE mydb"#,
        r#"DROP DATABASE IF EXISTS mydb WITH (FORCE)"#,
    ]);
}
