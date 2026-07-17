mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER INDEX
// Description: change the definition of an index
// Syntax:
// ALTER INDEX [ IF EXISTS ] name RENAME TO new_name
// ALTER INDEX [ IF EXISTS ] name SET TABLESPACE tablespace_name
// ALTER INDEX name ATTACH PARTITION index_name
// ALTER INDEX name [ NO ] DEPENDS ON EXTENSION extension_name
// ALTER INDEX [ IF EXISTS ] name SET ( storage_parameter [= value] [, ... ] )
// ALTER INDEX [ IF EXISTS ] name RESET ( storage_parameter [, ... ] )
// ALTER INDEX [ IF EXISTS ] name ALTER [ COLUMN ] column_number
//     SET STATISTICS integer
// ALTER INDEX ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
//     SET TABLESPACE new_tablespace [ NOWAIT ]
//
// URL: https://www.postgresql.org/docs/18/sql-alterindex.html

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
