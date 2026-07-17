mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_view_parses() {
    run_cases(&[
        r#"ALTER VIEW IF EXISTS v ALTER COLUMN col SET DEFAULT 1"#,
        r#"ALTER VIEW v ALTER COLUMN col DROP DEFAULT"#,
        r#"ALTER VIEW v OWNER TO CURRENT_USER"#,
        r#"ALTER VIEW v RENAME TO v_new"#,
        r#"ALTER VIEW v SET SCHEMA public"#,
        r#"ALTER VIEW v SET (security_barrier = true, check_option = local)"#,
        r#"ALTER VIEW v RESET (security_barrier)"#,
    ]);
}
