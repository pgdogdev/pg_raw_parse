mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER SYSTEM
// Description: change a server configuration parameter
// Syntax:
// ALTER SYSTEM SET configuration_parameter { TO | = } { value [, ...] | DEFAULT }
//
// ALTER SYSTEM RESET configuration_parameter
// ALTER SYSTEM RESET ALL
//
// URL: https://www.postgresql.org/docs/18/sql-altersystem.html

#[test]
fn alter_system_parses() {
    run_cases(&[
        r#"ALTER SYSTEM SET work_mem TO '64MB'"#,
        r#"ALTER SYSTEM SET work_mem = '64MB'"#,
        r#"ALTER SYSTEM RESET work_mem"#,
        r#"ALTER SYSTEM RESET ALL"#,
    ]);
}
