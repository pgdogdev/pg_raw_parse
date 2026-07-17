mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TRIGGER
// Description: change the definition of a trigger
// Syntax:
// ALTER TRIGGER name ON table_name RENAME TO new_name
// ALTER TRIGGER name ON table_name [ NO ] DEPENDS ON EXTENSION extension_name
//
// URL: https://www.postgresql.org/docs/18/sql-altertrigger.html

#[test]
fn alter_trigger_parses() {
    run_cases(&[
        r#"ALTER TRIGGER trg ON my_table RENAME TO trg_new"#,
        r#"ALTER TRIGGER trg ON my_table DEPENDS ON EXTENSION hstore"#,
        r#"ALTER TRIGGER trg ON my_table NO DEPENDS ON EXTENSION hstore"#,
    ]);
}
