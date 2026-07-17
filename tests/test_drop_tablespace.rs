mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TABLESPACE
// Description: remove a tablespace
// Syntax:
// DROP TABLESPACE [ IF EXISTS ] name
//
// URL: https://www.postgresql.org/docs/18/sql-droptablespace.html

#[test]
fn drop_tablespace_parses() {
    run_cases(&[
        r#"DROP TABLESPACE fastspace"#,
        r#"DROP TABLESPACE IF EXISTS fastspace"#,
    ]);
}
