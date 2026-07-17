mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE INDEX
// Description: define a new index
// Syntax:
// CREATE [ UNIQUE ] INDEX [ CONCURRENTLY ] [ [ IF NOT EXISTS ] name ] ON [ ONLY ] table_name [ USING method ]
//     ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass [ ( opclass_parameter = value [, ... ] ) ] ] [ ASC | DESC ] [ NULLS { FIRST | LAST } ] [, ...] )
//     [ INCLUDE ( column_name [, ...] ) ]
//     [ NULLS [ NOT ] DISTINCT ]
//     [ WITH ( storage_parameter [= value] [, ... ] ) ]
//     [ TABLESPACE tablespace_name ]
//     [ WHERE predicate ]
//
// URL: https://www.postgresql.org/docs/18/sql-createindex.html

#[test]
fn create_index_parses() {
    run_cases(&[
        r#"CREATE INDEX my_index ON my_table (id)"#,
        r#"CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS my_index ON ONLY my_table USING btree (id COLLATE "C" int4_ops ASC NULLS LAST, (lower(name)) DESC) INCLUDE (email) NULLS NOT DISTINCT WITH (fillfactor = 80) TABLESPACE fastspace WHERE id > 0"#,
    ]);
}
