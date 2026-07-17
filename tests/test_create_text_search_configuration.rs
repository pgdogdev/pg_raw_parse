mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TEXT SEARCH CONFIGURATION
// Description: define a new text search configuration
// Syntax:
// CREATE TEXT SEARCH CONFIGURATION name (
//     PARSER = parser_name |
//     COPY = source_config
// )
//
// URL: https://www.postgresql.org/docs/18/sql-createtsconfig.html

#[test]
fn create_text_search_configuration_parses() {
    run_cases(&[
        r#"CREATE TEXT SEARCH CONFIGURATION my_config (PARSER = default)"#,
        r#"CREATE TEXT SEARCH CONFIGURATION my_config (COPY = pg_catalog.english)"#,
    ]);
}
