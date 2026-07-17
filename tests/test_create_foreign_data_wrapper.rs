mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE FOREIGN DATA WRAPPER
// Description: define a new foreign-data wrapper
// Syntax:
// CREATE FOREIGN DATA WRAPPER name
//     [ HANDLER handler_function | NO HANDLER ]
//     [ VALIDATOR validator_function | NO VALIDATOR ]
//     [ OPTIONS ( option 'value' [, ... ] ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-createforeigndatawrapper.html

#[test]
fn create_foreign_data_wrapper_parses() {
    run_cases(&[
        r#"CREATE FOREIGN DATA WRAPPER my_fdw HANDLER my_handler VALIDATOR my_validator OPTIONS (host 'localhost')"#,
        r#"CREATE FOREIGN DATA WRAPPER my_fdw NO HANDLER NO VALIDATOR"#,
    ]);
}
