mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn reindex_parses() {
    run_cases(&[
        r#"REINDEX INDEX my_index"#,
        r#"REINDEX TABLE CONCURRENTLY my_table"#,
        r#"REINDEX SCHEMA public"#,
        r#"REINDEX DATABASE mydb"#,
        r#"REINDEX SYSTEM mydb"#,
        r#"REINDEX (VERBOSE true, TABLESPACE fastspace) INDEX CONCURRENTLY my_index"#,
    ]);
}
