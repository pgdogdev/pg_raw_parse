mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE LANGUAGE
// Description: define a new procedural language
// Syntax:
// CREATE [ OR REPLACE ] [ TRUSTED ] [ PROCEDURAL ] LANGUAGE name
//     HANDLER call_handler [ INLINE inline_handler ] [ VALIDATOR valfunction ]
// CREATE [ OR REPLACE ] [ TRUSTED ] [ PROCEDURAL ] LANGUAGE name
//
// URL: https://www.postgresql.org/docs/18/sql-createlanguage.html

#[test]
fn create_language_parses() {
    run_cases(&[
        r#"CREATE LANGUAGE plsample"#,
        r#"CREATE OR REPLACE TRUSTED PROCEDURAL LANGUAGE plsample HANDLER my_handler INLINE my_inline VALIDATOR my_validator"#,
    ]);
}
