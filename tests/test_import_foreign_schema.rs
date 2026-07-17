mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     IMPORT FOREIGN SCHEMA
// Description: import table definitions from a foreign server
// Syntax:
// IMPORT FOREIGN SCHEMA remote_schema
//     [ { LIMIT TO | EXCEPT } ( table_name [, ...] ) ]
//     FROM SERVER server_name
//     INTO local_schema
//     [ OPTIONS ( option 'value' [, ... ] ) ]
//
// URL: https://www.postgresql.org/docs/18/sql-importforeignschema.html

#[test]
fn import_foreign_schema_parses() {
    run_cases(&[
        r#"IMPORT FOREIGN SCHEMA public FROM SERVER my_server INTO local_schema"#,
        r#"IMPORT FOREIGN SCHEMA public LIMIT TO (t1, t2) FROM SERVER my_server INTO local_schema OPTIONS (import_default 'true')"#,
        r#"IMPORT FOREIGN SCHEMA public EXCEPT (t1, t2) FROM SERVER my_server INTO local_schema"#,
    ]);
}
