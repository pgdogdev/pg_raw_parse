mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE STATISTICS
// Description: define extended statistics
// Syntax:
// CREATE STATISTICS [ [ IF NOT EXISTS ] statistics_name ]
//     ON ( expression )
//     FROM table_name
//
// CREATE STATISTICS [ [ IF NOT EXISTS ] statistics_name ]
//     [ ( statistics_kind [, ... ] ) ]
//     ON { column_name | ( expression ) }, { column_name | ( expression ) } [, ...]
//     FROM table_name
//
// URL: https://www.postgresql.org/docs/18/sql-createstatistics.html

#[test]
fn create_statistics_parses() {
    run_cases(&[
        r#"CREATE STATISTICS stats ON id, name FROM users"#,
        r#"CREATE STATISTICS IF NOT EXISTS stats (dependencies, ndistinct, mcv) ON id, lower(name) FROM users"#,
    ]);
}
