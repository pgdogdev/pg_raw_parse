mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TEXT SEARCH PARSER
// Description: change the definition of a text search parser
// Syntax:
// ALTER TEXT SEARCH PARSER name RENAME TO new_name
// ALTER TEXT SEARCH PARSER name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-altertsparser.html

#[test]
fn alter_text_search_parser_parses() {
    run_cases(&[
        r#"ALTER TEXT SEARCH PARSER my_parser RENAME TO my_parser_new"#,
        r#"ALTER TEXT SEARCH PARSER my_parser SET SCHEMA public"#,
    ]);
}
