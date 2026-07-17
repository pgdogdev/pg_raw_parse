mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_server_parses() {
    run_cases(&[
        r#"ALTER SERVER my_server VERSION '2.0' OPTIONS (ADD host 'localhost', SET port '5432', DROP oldopt)"#,
        r#"ALTER SERVER my_server OWNER TO CURRENT_USER"#,
        r#"ALTER SERVER my_server RENAME TO my_server_new"#,
    ]);
}
