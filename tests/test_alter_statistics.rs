mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER STATISTICS
// Description: change the definition of an extended statistics object
// Syntax:
// ALTER STATISTICS name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER STATISTICS name RENAME TO new_name
// ALTER STATISTICS name SET SCHEMA new_schema
// ALTER STATISTICS name SET STATISTICS { new_target | DEFAULT }
//
// URL: https://www.postgresql.org/docs/18/sql-alterstatistics.html

#[test]
fn alter_statistics_parses() {
    run_cases(&[
        r#"ALTER STATISTICS stats SET STATISTICS 1000"#,
        r#"ALTER STATISTICS stats OWNER TO CURRENT_USER"#,
        r#"ALTER STATISTICS stats RENAME TO stats_new"#,
        r#"ALTER STATISTICS stats SET SCHEMA public"#,
    ]);
}
