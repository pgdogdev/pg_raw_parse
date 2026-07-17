mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE SEQUENCE
// Description: define a new sequence generator
// Syntax:
// CREATE [ { TEMPORARY | TEMP } | UNLOGGED ] SEQUENCE [ IF NOT EXISTS ] name
//     [ AS data_type ]
//     [ INCREMENT [ BY ] increment ]
//     [ MINVALUE minvalue | NO MINVALUE ] [ MAXVALUE maxvalue | NO MAXVALUE ]
//     [ [ NO ] CYCLE ]
//     [ START [ WITH ] start ]
//     [ CACHE cache ]
//     [ OWNED BY { table_name.column_name | NONE } ]
//
// URL: https://www.postgresql.org/docs/18/sql-createsequence.html

#[test]
fn create_sequence_parses() {
    run_cases(&[
        r#"CREATE SEQUENCE seq"#,
        r#"CREATE TEMPORARY SEQUENCE IF NOT EXISTS seq AS bigint INCREMENT BY 2 MINVALUE 1 MAXVALUE 100 START WITH 10 CACHE 5 CYCLE OWNED BY users.id"#,
    ]);
}
