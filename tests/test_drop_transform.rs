mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP TRANSFORM
// Description: remove a transform
// Syntax:
// DROP TRANSFORM [ IF EXISTS ] FOR type_name LANGUAGE lang_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptransform.html

#[test]
fn drop_transform_parses() {
    run_cases(&[
        r#"DROP TRANSFORM FOR my_type LANGUAGE plsample"#,
        r#"DROP TRANSFORM IF EXISTS FOR my_type LANGUAGE plsample CASCADE"#,
    ]);
}
