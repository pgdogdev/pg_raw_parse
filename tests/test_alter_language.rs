mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER LANGUAGE
// Description: change the definition of a procedural language
// Syntax:
// ALTER [ PROCEDURAL ] LANGUAGE name RENAME TO new_name
// ALTER [ PROCEDURAL ] LANGUAGE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// URL: https://www.postgresql.org/docs/18/sql-alterlanguage.html

#[test]
fn alter_language_parses() {
    run_cases(&[
        r#"ALTER LANGUAGE plsample RENAME TO plsample2"#,
        r#"ALTER LANGUAGE plsample OWNER TO CURRENT_USER"#,
    ]);
}
