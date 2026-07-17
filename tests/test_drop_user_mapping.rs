mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_user_mapping_parses() {
    run_cases(&[
        r#"DROP USER MAPPING FOR app_user SERVER my_server"#,
        r#"DROP USER MAPPING IF EXISTS FOR CURRENT_USER SERVER my_server"#,
        r#"DROP USER MAPPING IF EXISTS FOR PUBLIC SERVER my_server"#,
    ]);
}
