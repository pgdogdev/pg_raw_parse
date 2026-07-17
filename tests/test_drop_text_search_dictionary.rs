mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TEXT SEARCH DICTIONARY
// Description: remove a text search dictionary
// Syntax:
// DROP TEXT SEARCH DICTIONARY [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptsdictionary.html

#[test]
fn drop_text_search_dictionary_parses() {
    run_cases(&[
        r#"DROP TEXT SEARCH DICTIONARY my_dict"#,
        r#"DROP TEXT SEARCH DICTIONARY IF EXISTS my_dict, my_dict_old CASCADE"#,
        r#"DROP TEXT SEARCH DICTIONARY IF EXISTS my_dict RESTRICT"#,
    ]);
}
