mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_tablespace_parses() {
    run_cases(&[
        r#"ALTER TABLESPACE fastspace RENAME TO slowspace"#,
        r#"ALTER TABLESPACE fastspace OWNER TO CURRENT_USER"#,
        r#"ALTER TABLESPACE fastspace SET (random_page_cost = 1.1, seq_page_cost = 1.0)"#,
        r#"ALTER TABLESPACE fastspace RESET (random_page_cost)"#,
    ]);
}
