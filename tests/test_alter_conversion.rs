mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_conversion_parses() {
    run_cases(&[
        r#"ALTER CONVERSION my_conversion RENAME TO my_new_conversion"#,
        r#"ALTER CONVERSION my_conversion OWNER TO CURRENT_USER"#,
        r#"ALTER CONVERSION my_conversion SET SCHEMA public"#,
    ]);
}
