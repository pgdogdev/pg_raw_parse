mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_text_search_configuration_parses() {
    run_cases(&[
        r#"CREATE TEXT SEARCH CONFIGURATION my_config (PARSER = default)"#,
        r#"CREATE TEXT SEARCH CONFIGURATION my_config (COPY = pg_catalog.english)"#,
    ]);
}
