mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP FOREIGN DATA WRAPPER
// Description: remove a foreign-data wrapper
// Syntax:
// DROP FOREIGN DATA WRAPPER [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropforeigndatawrapper.html

#[test]
fn drop_foreign_data_wrapper_parses() {
    run_cases(&[
        r#"DROP FOREIGN DATA WRAPPER my_fdw"#,
        r#"DROP FOREIGN DATA WRAPPER IF EXISTS my_fdw, my_fdw_old CASCADE"#,
        r#"DROP FOREIGN DATA WRAPPER IF EXISTS my_fdw RESTRICT"#,
    ]);
}
