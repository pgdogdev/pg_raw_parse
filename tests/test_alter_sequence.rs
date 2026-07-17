mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_sequence_parses() {
    run_cases(&[
        r#"ALTER SEQUENCE IF EXISTS seq AS bigint INCREMENT BY 2 MINVALUE 1 MAXVALUE 1000 START WITH 10 RESTART WITH 20 CACHE 5 CYCLE OWNED BY users.id"#,
        r#"ALTER SEQUENCE seq NO MINVALUE NO MAXVALUE NO CYCLE OWNED BY NONE"#,
        r#"ALTER SEQUENCE seq OWNER TO CURRENT_USER"#,
        r#"ALTER SEQUENCE seq RENAME TO seq_new"#,
        r#"ALTER SEQUENCE seq SET SCHEMA public"#,
    ]);
}
