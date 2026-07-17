mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_sequence_parses() {
    run_cases(&[
        r#"CREATE SEQUENCE seq"#,
        r#"CREATE TEMPORARY SEQUENCE IF NOT EXISTS seq AS bigint INCREMENT BY 2 MINVALUE 1 MAXVALUE 100 START WITH 10 CACHE 5 CYCLE OWNED BY users.id"#,
    ]);
}
