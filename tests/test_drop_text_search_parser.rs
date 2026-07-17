mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_text_search_parser_parses() {
    run_cases(&[
        r#"DROP TEXT SEARCH PARSER my_parser"#,
        r#"DROP TEXT SEARCH PARSER IF EXISTS my_parser, my_parser_old CASCADE"#,
        r#"DROP TEXT SEARCH PARSER IF EXISTS my_parser RESTRICT"#,
    ]);
}
