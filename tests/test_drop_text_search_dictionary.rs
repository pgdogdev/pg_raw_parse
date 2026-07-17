mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_text_search_dictionary_parses() {
    run_cases(&[
        r#"DROP TEXT SEARCH DICTIONARY my_dict"#,
        r#"DROP TEXT SEARCH DICTIONARY IF EXISTS my_dict, my_dict_old CASCADE"#,
        r#"DROP TEXT SEARCH DICTIONARY IF EXISTS my_dict RESTRICT"#,
    ]);
}
