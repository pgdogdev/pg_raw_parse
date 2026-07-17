mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TEXT SEARCH TEMPLATE
// Description: define a new text search template
// Syntax:
// CREATE TEXT SEARCH TEMPLATE name (
//     [ INIT = init_function , ]
//     LEXIZE = lexize_function
// )
//
// URL: https://www.postgresql.org/docs/18/sql-createtstemplate.html

#[test]
fn create_text_search_template_parses() {
    run_cases(&[
        r#"CREATE TEXT SEARCH TEMPLATE my_template (INIT = dsimple_init, LEXIZE = dsimple_lexize)"#,
        r#"CREATE TEXT SEARCH TEMPLATE my_template (LEXIZE = dsimple_lexize)"#,
    ]);
}
