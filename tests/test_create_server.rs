mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_server_parses() {
    run_cases(&[
        r#"CREATE SERVER my_server FOREIGN DATA WRAPPER my_fdw"#,
        r#"CREATE SERVER IF NOT EXISTS my_server TYPE 'postgres' VERSION '18' FOREIGN DATA WRAPPER my_fdw OPTIONS (host 'localhost', dbname 'postgres')"#,
    ]);
}
