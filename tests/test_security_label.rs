mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn security_label_parses() {
    run_cases(&[
        r#"SECURITY LABEL ON TABLE my_table IS 'classified'"#,
        r#"SECURITY LABEL FOR selinux ON COLUMN my_table.id IS 'system_u:object_r:sepgsql_table_t:s0'"#,
        r#"SECURITY LABEL ON FUNCTION my_function(integer) IS NULL"#,
    ]);
}
