mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn import_foreign_schema_parses() {
    run_cases(&[
        r#"IMPORT FOREIGN SCHEMA public FROM SERVER my_server INTO local_schema"#,
        r#"IMPORT FOREIGN SCHEMA public LIMIT TO (t1, t2) FROM SERVER my_server INTO local_schema OPTIONS (import_default 'true')"#,
        r#"IMPORT FOREIGN SCHEMA public EXCEPT (t1, t2) FROM SERVER my_server INTO local_schema"#,
    ]);
}
