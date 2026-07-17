mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP LANGUAGE
// Description: remove a procedural language
// Syntax:
// DROP [ PROCEDURAL ] LANGUAGE [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droplanguage.html

#[test]
fn drop_language_parses() {
    run_cases(&[
        r#"DROP LANGUAGE plsample"#,
        r#"DROP LANGUAGE IF EXISTS plsample, plsample_old CASCADE"#,
        r#"DROP LANGUAGE IF EXISTS plsample RESTRICT"#,
    ]);
}
