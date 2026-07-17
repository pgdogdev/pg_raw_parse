mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_language_parses() {
    run_cases(&[
        r#"ALTER LANGUAGE plsample RENAME TO plsample2"#,
        r#"ALTER LANGUAGE plsample OWNER TO CURRENT_USER"#,
    ]);
}
