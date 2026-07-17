mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TEXT SEARCH PARSER
// Description: define a new text search parser
// Syntax:
// CREATE TEXT SEARCH PARSER name (
//     START = start_function ,
//     GETTOKEN = gettoken_function ,
//     END = end_function ,
//     LEXTYPES = lextypes_function
//     [, HEADLINE = headline_function ]
// )
//
// URL: https://www.postgresql.org/docs/18/sql-createtsparser.html

#[test]
fn create_text_search_parser_parses() {
    run_cases(&[
        r#"CREATE TEXT SEARCH PARSER my_parser (START = prsd_start, GETTOKEN = prsd_nexttoken, END = prsd_end, LEXTYPES = prsd_lextype, HEADLINE = prsd_headline)"#,
    ]);
}
