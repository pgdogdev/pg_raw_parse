mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TEXT SEARCH CONFIGURATION
// Description: remove a text search configuration
// Syntax:
// DROP TEXT SEARCH CONFIGURATION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptsconfig.html

#[test]
fn drop_text_search_configuration_parses() {
    run_cases(&[
        r#"DROP TEXT SEARCH CONFIGURATION my_config"#,
        r#"DROP TEXT SEARCH CONFIGURATION IF EXISTS my_config, my_config_old CASCADE"#,
        r#"DROP TEXT SEARCH CONFIGURATION IF EXISTS my_config RESTRICT"#,
    ]);
}
