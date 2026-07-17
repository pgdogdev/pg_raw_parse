mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP EXTENSION
// Description: remove an extension
// Syntax:
// DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropextension.html

#[test]
fn drop_extension_parses() {
    run_cases(&[
        r#"DROP EXTENSION hstore"#,
        r#"DROP EXTENSION IF EXISTS hstore, hstore_old CASCADE"#,
        r#"DROP EXTENSION IF EXISTS hstore RESTRICT"#,
    ]);
}
