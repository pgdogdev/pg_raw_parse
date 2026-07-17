mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_text_search_configuration_parses() {
    run_cases(&[
        r#"DROP TEXT SEARCH CONFIGURATION my_config"#,
        r#"DROP TEXT SEARCH CONFIGURATION IF EXISTS my_config, my_config_old CASCADE"#,
        r#"DROP TEXT SEARCH CONFIGURATION IF EXISTS my_config RESTRICT"#,
    ]);
}
