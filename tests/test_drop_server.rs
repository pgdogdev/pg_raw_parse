mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP SERVER
// Description: remove a foreign server descriptor
// Syntax:
// DROP SERVER [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropserver.html

#[test]
fn drop_server_parses() {
    run_cases(&[
        r#"DROP SERVER my_server"#,
        r#"DROP SERVER IF EXISTS my_server, my_server_old CASCADE"#,
        r#"DROP SERVER IF EXISTS my_server RESTRICT"#,
    ]);
}
