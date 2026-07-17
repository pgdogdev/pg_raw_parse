mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_trigger_parses() {
    run_cases(&[
        r#"ALTER TRIGGER trg ON my_table RENAME TO trg_new"#,
        r#"ALTER TRIGGER trg ON my_table DEPENDS ON EXTENSION hstore"#,
        r#"ALTER TRIGGER trg ON my_table NO DEPENDS ON EXTENSION hstore"#,
    ]);
}
