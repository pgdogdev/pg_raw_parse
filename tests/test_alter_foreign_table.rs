mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER FOREIGN TABLE
// Description: change the definition of a foreign table
// Syntax:
// ALTER FOREIGN TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     action [, ... ]
// ALTER FOREIGN TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     RENAME [ COLUMN ] column_name TO new_column_name
// ALTER FOREIGN TABLE [ IF EXISTS ] name
//     RENAME TO new_name
// ALTER FOREIGN TABLE [ IF EXISTS ] name
//     SET SCHEMA new_schema
//
// where action is one of:
//
//     ADD [ COLUMN ] [ IF NOT EXISTS ] column_name data_type [ COLLATE collation ] [ column_constraint [ ... ] ]
//     DROP [ COLUMN ] [ IF EXISTS ] column_name [ RESTRICT | CASCADE ]
//     ALTER [ COLUMN ] column_name [ SET DATA ] TYPE data_type [ COLLATE collation ]
//     ALTER [ COLUMN ] column_name SET DEFAULT expression
//     ALTER [ COLUMN ] column_name DROP DEFAULT
//     ALTER [ COLUMN ] column_name { SET | DROP } NOT NULL
//     ALTER [ COLUMN ] column_name SET STATISTICS integer
//     ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
//     ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
//     ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
//     ALTER [ COLUMN ] column_name OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ])
//     ADD table_constraint [ NOT VALID ]
//     VALIDATE CONSTRAINT constraint_name
//     DROP CONSTRAINT [ IF EXISTS ]  constraint_name [ RESTRICT | CASCADE ]
//     DISABLE TRIGGER [ trigger_name | ALL | USER ]
//     ENABLE TRIGGER [ trigger_name | ALL | USER ]
//     ENABLE REPLICA TRIGGER trigger_name
//     ENABLE ALWAYS TRIGGER trigger_name
//     SET WITHOUT OIDS
//     INHERIT parent_table
//     NO INHERIT parent_table
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//     OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ])
//
// URL: https://www.postgresql.org/docs/18/sql-alterforeigntable.html

#[test]
fn alter_foreign_table_parses() {
    run_cases(&[
        r#"ALTER FOREIGN TABLE IF EXISTS ONLY ft ADD COLUMN IF NOT EXISTS payload text OPTIONS (column_name 'payload')"#,
        r#"ALTER FOREIGN TABLE ft DROP COLUMN IF EXISTS old_col CASCADE"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload TYPE varchar(200)"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload SET DEFAULT 'x'"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload DROP DEFAULT"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload SET NOT NULL"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload DROP NOT NULL"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload SET STATISTICS 100"#,
        r#"ALTER FOREIGN TABLE ft ADD CHECK (payload <> '') NOT VALID"#,
        r#"ALTER FOREIGN TABLE ft VALIDATE CONSTRAINT payload_check"#,
        r#"ALTER FOREIGN TABLE ft DROP CONSTRAINT IF EXISTS payload_check RESTRICT"#,
        r#"ALTER FOREIGN TABLE ft DISABLE TRIGGER ALL"#,
        r#"ALTER FOREIGN TABLE ft ENABLE ALWAYS TRIGGER trg"#,
        r#"ALTER FOREIGN TABLE ft SET WITHOUT OIDS"#,
        r#"ALTER FOREIGN TABLE ft OWNER TO CURRENT_USER"#,
        r#"ALTER FOREIGN TABLE ft OPTIONS (ADD schema_name 'public', SET table_name 'remote_table')"#,
        r#"ALTER FOREIGN TABLE ft RENAME COLUMN payload TO body"#,
        r#"ALTER FOREIGN TABLE ft RENAME TO ft_new"#,
        r#"ALTER FOREIGN TABLE ft SET SCHEMA archive"#,
    ]);
}
