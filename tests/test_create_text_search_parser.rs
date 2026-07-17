mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_text_search_parser_parses() {
    run_cases(&[
        r#"CREATE TEXT SEARCH PARSER my_parser (START = prsd_start, GETTOKEN = prsd_nexttoken, END = prsd_end, LEXTYPES = prsd_lextype, HEADLINE = prsd_headline)"#,
    ]);
}
