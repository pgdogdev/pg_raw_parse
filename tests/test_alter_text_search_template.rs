mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TEXT SEARCH TEMPLATE
// Description: change the definition of a text search template
// Syntax:
// ALTER TEXT SEARCH TEMPLATE name RENAME TO new_name
// ALTER TEXT SEARCH TEMPLATE name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-altertstemplate.html

#[test]
fn alter_text_search_template_parses() {
    run_cases(&[
        r#"ALTER TEXT SEARCH TEMPLATE my_template RENAME TO my_template_new"#,
        r#"ALTER TEXT SEARCH TEMPLATE my_template SET SCHEMA public"#,
    ]);
}
