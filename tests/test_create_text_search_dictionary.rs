mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TEXT SEARCH DICTIONARY
// Description: define a new text search dictionary
// Syntax:
// CREATE TEXT SEARCH DICTIONARY name (
//     TEMPLATE = template
//     [, option = value [, ... ]]
// )
//
// URL: https://www.postgresql.org/docs/18/sql-createtsdictionary.html

#[test]
fn create_text_search_dictionary_parses() {
    run_cases(&[
        r#"CREATE TEXT SEARCH DICTIONARY my_dict (TEMPLATE = simple, StopWords = english)"#,
    ]);
}
