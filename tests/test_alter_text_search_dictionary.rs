mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_text_search_dictionary_parses() {
    run_cases(&[
        r#"ALTER TEXT SEARCH DICTIONARY my_dict (StopWords = english)"#,
        r#"ALTER TEXT SEARCH DICTIONARY my_dict RENAME TO my_dict_new"#,
        r#"ALTER TEXT SEARCH DICTIONARY my_dict OWNER TO CURRENT_USER"#,
        r#"ALTER TEXT SEARCH DICTIONARY my_dict SET SCHEMA public"#,
    ]);
}
