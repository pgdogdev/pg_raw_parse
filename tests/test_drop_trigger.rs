mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TRIGGER
// Description: remove a trigger
// Syntax:
// DROP TRIGGER [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptrigger.html

#[test]
fn drop_trigger_parses() {
    run_cases(&[
        r#"DROP TRIGGER trg ON my_table"#,
        r#"DROP TRIGGER IF EXISTS trg ON my_table CASCADE"#,
    ]);
}
