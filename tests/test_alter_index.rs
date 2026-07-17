mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_index_parses() {
    run_cases(&[
        r#"ALTER INDEX IF EXISTS my_index RENAME TO my_index_new"#,
        r#"ALTER INDEX ALL IN TABLESPACE oldspace OWNED BY app_user SET TABLESPACE newspace NOWAIT"#,
        r#"ALTER INDEX my_index ATTACH PARTITION my_index_part"#,
        r#"ALTER INDEX my_index DEPENDS ON EXTENSION hstore"#,
        r#"ALTER INDEX my_index NO DEPENDS ON EXTENSION hstore"#,
        r#"ALTER INDEX my_index SET TABLESPACE fastspace"#,
        r#"ALTER INDEX my_index SET (fillfactor = 80, deduplicate_items = off)"#,
        r#"ALTER INDEX my_index RESET (fillfactor)"#,
        r#"ALTER INDEX my_index ALTER COLUMN 1 SET STATISTICS 100"#,
    ]);
}
