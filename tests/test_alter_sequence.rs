mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER SEQUENCE
// Description: change the definition of a sequence generator
// Syntax:
// ALTER SEQUENCE [ IF EXISTS ] name
//     [ AS data_type ]
//     [ INCREMENT [ BY ] increment ]
//     [ MINVALUE minvalue | NO MINVALUE ] [ MAXVALUE maxvalue | NO MAXVALUE ]
//     [ [ NO ] CYCLE ]
//     [ START [ WITH ] start ]
//     [ RESTART [ [ WITH ] restart ] ]
//     [ CACHE cache ]
//     [ OWNED BY { table_name.column_name | NONE } ]
// ALTER SEQUENCE [ IF EXISTS ] name SET { LOGGED | UNLOGGED }
// ALTER SEQUENCE [ IF EXISTS ] name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER SEQUENCE [ IF EXISTS ] name RENAME TO new_name
// ALTER SEQUENCE [ IF EXISTS ] name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-altersequence.html

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
