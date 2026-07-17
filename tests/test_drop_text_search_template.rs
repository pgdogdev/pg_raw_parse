mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TEXT SEARCH TEMPLATE
// Description: remove a text search template
// Syntax:
// DROP TEXT SEARCH TEMPLATE [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptstemplate.html

#[test]
fn drop_text_search_template_parses() {
    run_cases(&[
        r#"DROP TEXT SEARCH TEMPLATE my_template"#,
        r#"DROP TEXT SEARCH TEMPLATE IF EXISTS my_template, my_template_old CASCADE"#,
        r#"DROP TEXT SEARCH TEMPLATE IF EXISTS my_template RESTRICT"#,
    ]);
}
