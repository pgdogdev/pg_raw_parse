mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn comment_parses() {
    run_cases(&[
        r#"COMMENT ON TABLE my_table IS 'table comment'"#,
        r#"COMMENT ON COLUMN my_table.id IS 'column comment'"#,
        r#"COMMENT ON FUNCTION my_function(integer) IS 'function comment'"#,
        r#"COMMENT ON OPERATOR +(integer, integer) IS 'operator comment'"#,
        r#"COMMENT ON CONSTRAINT my_constraint ON my_table IS 'constraint comment'"#,
        r#"COMMENT ON LARGE OBJECT 12345 IS 'large object comment'"#,
        r#"COMMENT ON TABLE my_table IS NULL"#,
    ]);
}
