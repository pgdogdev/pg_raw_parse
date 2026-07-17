mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP GROUP
// Description: remove a database role
// Syntax:
// DROP GROUP [ IF EXISTS ] name [, ...]
//
// URL: https://www.postgresql.org/docs/18/sql-dropgroup.html

#[test]
fn drop_group_parses() {
    run_cases(&[
        r#"DROP GROUP my_group"#,
        r#"DROP GROUP IF EXISTS my_group, my_group_old"#,
    ]);
}
