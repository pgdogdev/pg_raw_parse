mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE EXTENSION
// Description: install an extension
// Syntax:
// CREATE EXTENSION [ IF NOT EXISTS ] extension_name
//     [ WITH ] [ SCHEMA schema_name ]
//              [ VERSION version ]
//              [ CASCADE ]
//
// URL: https://www.postgresql.org/docs/18/sql-createextension.html

#[test]
fn create_extension_parses() {
    run_cases(&[
        r#"CREATE EXTENSION hstore"#,
        r#"CREATE EXTENSION IF NOT EXISTS hstore WITH SCHEMA public VERSION '1.8' CASCADE"#,
    ]);
}
