mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_server_parses() {
    run_cases(&[
        r#"DROP SERVER my_server"#,
        r#"DROP SERVER IF EXISTS my_server, my_server_old CASCADE"#,
        r#"DROP SERVER IF EXISTS my_server RESTRICT"#,
    ]);
}
