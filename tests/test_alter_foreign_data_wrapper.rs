mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER FOREIGN DATA WRAPPER
// Description: change the definition of a foreign-data wrapper
// Syntax:
// ALTER FOREIGN DATA WRAPPER name
//     [ HANDLER handler_function | NO HANDLER ]
//     [ VALIDATOR validator_function | NO VALIDATOR ]
//     [ OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ]) ]
// ALTER FOREIGN DATA WRAPPER name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER FOREIGN DATA WRAPPER name RENAME TO new_name
//
// URL: https://www.postgresql.org/docs/18/sql-alterforeigndatawrapper.html

#[test]
fn alter_foreign_data_wrapper_parses() {
    run_cases(&[
        r#"ALTER FOREIGN DATA WRAPPER my_fdw HANDLER my_handler"#,
        r#"ALTER FOREIGN DATA WRAPPER my_fdw NO HANDLER"#,
        r#"ALTER FOREIGN DATA WRAPPER my_fdw VALIDATOR my_validator"#,
        r#"ALTER FOREIGN DATA WRAPPER my_fdw NO VALIDATOR"#,
        r#"ALTER FOREIGN DATA WRAPPER my_fdw OPTIONS (ADD host 'localhost', SET port '5432', DROP oldopt)"#,
        r#"ALTER FOREIGN DATA WRAPPER my_fdw OWNER TO CURRENT_USER"#,
        r#"ALTER FOREIGN DATA WRAPPER my_fdw RENAME TO my_fdw_new"#,
    ]);
}
