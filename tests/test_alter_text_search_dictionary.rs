mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TEXT SEARCH DICTIONARY
// Description: change the definition of a text search dictionary
// Syntax:
// ALTER TEXT SEARCH DICTIONARY name (
//     option [ = value ] [, ... ]
// )
// ALTER TEXT SEARCH DICTIONARY name RENAME TO new_name
// ALTER TEXT SEARCH DICTIONARY name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TEXT SEARCH DICTIONARY name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-altertsdictionary.html

#[test]
fn alter_text_search_dictionary_parses() {
    run_cases(&[
        r#"ALTER TEXT SEARCH DICTIONARY my_dict (StopWords = english)"#,
        r#"ALTER TEXT SEARCH DICTIONARY my_dict RENAME TO my_dict_new"#,
        r#"ALTER TEXT SEARCH DICTIONARY my_dict OWNER TO CURRENT_USER"#,
        r#"ALTER TEXT SEARCH DICTIONARY my_dict SET SCHEMA public"#,
    ]);
}
