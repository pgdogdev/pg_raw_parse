mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE VIEW
// Description: define a new view
// Syntax:
// CREATE [ OR REPLACE ] [ TEMP | TEMPORARY ] [ RECURSIVE ] VIEW name [ ( column_name [, ...] ) ]
//     [ WITH ( view_option_name [= view_option_value] [, ... ] ) ]
//     AS query
//     [ WITH [ CASCADED | LOCAL ] CHECK OPTION ]
//
// URL: https://www.postgresql.org/docs/18/sql-createview.html

#[test]
fn create_view_parses() {
    run_cases(&[
        r#"CREATE VIEW v AS SELECT 1 AS col"#,
        r#"CREATE OR REPLACE TEMP RECURSIVE VIEW v (col) WITH (security_barrier = true, check_option = local) AS SELECT 1 AS col"#,
        r#"CREATE OR REPLACE TEMP VIEW v (col) WITH (security_barrier = true) AS SELECT 1 AS col WITH LOCAL CHECK OPTION"#,
        r#"CREATE VIEW v AS SELECT 1 AS col WITH CASCADED CHECK OPTION"#,
    ]);
}
