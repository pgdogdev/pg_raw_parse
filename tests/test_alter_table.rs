mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER TABLE
// Description: change the definition of a table
// Syntax:
// ALTER TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     action [, ... ]
// ALTER TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     RENAME [ COLUMN ] column_name TO new_column_name
// ALTER TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     RENAME CONSTRAINT constraint_name TO new_constraint_name
// ALTER TABLE [ IF EXISTS ] name
//     RENAME TO new_name
// ALTER TABLE [ IF EXISTS ] name
//     SET SCHEMA new_schema
// ALTER TABLE ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
//     SET TABLESPACE new_tablespace [ NOWAIT ]
// ALTER TABLE [ IF EXISTS ] name
//     ATTACH PARTITION partition_name { FOR VALUES partition_bound_spec | DEFAULT }
// ALTER TABLE [ IF EXISTS ] name
//     DETACH PARTITION partition_name [ CONCURRENTLY | FINALIZE ]
//
// where action is one of:
//
//     ADD [ COLUMN ] [ IF NOT EXISTS ] column_name data_type [ COLLATE collation ] [ column_constraint [ ... ] ]
//     DROP [ COLUMN ] [ IF EXISTS ] column_name [ RESTRICT | CASCADE ]
//     ALTER [ COLUMN ] column_name [ SET DATA ] TYPE data_type [ COLLATE collation ] [ USING expression ]
//     ALTER [ COLUMN ] column_name SET DEFAULT expression
//     ALTER [ COLUMN ] column_name DROP DEFAULT
//     ALTER [ COLUMN ] column_name { SET | DROP } NOT NULL
//     ALTER [ COLUMN ] column_name SET EXPRESSION AS ( expression )
//     ALTER [ COLUMN ] column_name DROP EXPRESSION [ IF EXISTS ]
//     ALTER [ COLUMN ] column_name ADD GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]
//     ALTER [ COLUMN ] column_name { SET GENERATED { ALWAYS | BY DEFAULT } | SET sequence_option | RESTART [ [ WITH ] restart ] } [...]
//     ALTER [ COLUMN ] column_name DROP IDENTITY [ IF EXISTS ]
//     ALTER [ COLUMN ] column_name SET STATISTICS { integer | DEFAULT }
//     ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
//     ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
//     ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
//     ALTER [ COLUMN ] column_name SET COMPRESSION compression_method
//     ADD table_constraint [ NOT VALID ]
//     ADD table_constraint_using_index
//     ALTER CONSTRAINT constraint_name [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ] [ ENFORCED | NOT ENFORCED ]
//     ALTER CONSTRAINT constraint_name [ INHERIT | NO INHERIT ]
//     VALIDATE CONSTRAINT constraint_name
//     DROP CONSTRAINT [ IF EXISTS ]  constraint_name [ RESTRICT | CASCADE ]
//     DISABLE TRIGGER [ trigger_name | ALL | USER ]
//     ENABLE TRIGGER [ trigger_name | ALL | USER ]
//     ENABLE REPLICA TRIGGER trigger_name
//     ENABLE ALWAYS TRIGGER trigger_name
//     DISABLE RULE rewrite_rule_name
//     ENABLE RULE rewrite_rule_name
//     ENABLE REPLICA RULE rewrite_rule_name
//     ENABLE ALWAYS RULE rewrite_rule_name
//     DISABLE ROW LEVEL SECURITY
//     ENABLE ROW LEVEL SECURITY
//     FORCE ROW LEVEL SECURITY
//     NO FORCE ROW LEVEL SECURITY
//     CLUSTER ON index_name
//     SET WITHOUT CLUSTER
//     SET WITHOUT OIDS
//     SET ACCESS METHOD { new_access_method | DEFAULT }
//     SET TABLESPACE new_tablespace
//     SET { LOGGED | UNLOGGED }
//     SET ( storage_parameter [= value] [, ... ] )
//     RESET ( storage_parameter [, ... ] )
//     INHERIT parent_table
//     NO INHERIT parent_table
//     OF type_name
//     NOT OF
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//     REPLICA IDENTITY { DEFAULT | USING INDEX index_name | FULL | NOTHING }
//
// and partition_bound_spec is:
//
// IN ( partition_bound_expr [, ...] ) |
// FROM ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] )
//   TO ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] ) |
// WITH ( MODULUS numeric_literal, REMAINDER numeric_literal )
//
// and column_constraint is:
//
// [ CONSTRAINT constraint_name ]
// { NOT NULL [ NO INHERIT ] |
//   NULL |
//   CHECK ( expression ) [ NO INHERIT ] |
//   DEFAULT default_expr |
//   GENERATED ALWAYS AS ( generation_expr ) [ STORED | VIRTUAL ] |
//   GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] index_parameters |
//   PRIMARY KEY index_parameters |
//   REFERENCES reftable [ ( refcolumn ) ] [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ]
//     [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
// [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ] [ ENFORCED | NOT ENFORCED ]
//
// and table_constraint is:
//
// [ CONSTRAINT constraint_name ]
// { CHECK ( expression ) [ NO INHERIT ] |
//   NOT NULL column_name [ NO INHERIT ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] ( column_name [, ... ] [, column_name WITHOUT OVERLAPS ] ) index_parameters |
//   PRIMARY KEY ( column_name [, ... ] [, column_name WITHOUT OVERLAPS ] ) index_parameters |
//   EXCLUDE [ USING index_method ] ( exclude_element WITH operator [, ... ] ) index_parameters [ WHERE ( predicate ) ] |
//   FOREIGN KEY ( column_name [, ... ] [, PERIOD column_name ] ) REFERENCES reftable [ ( refcolumn [, ... ]  [, PERIOD refcolumn ] ) ]
//     [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ] [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
// [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ] [ ENFORCED | NOT ENFORCED ]
//
// and table_constraint_using_index is:
//
//     [ CONSTRAINT constraint_name ]
//     { UNIQUE | PRIMARY KEY } USING INDEX index_name
//     [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
//
// index_parameters in UNIQUE, PRIMARY KEY, and EXCLUDE constraints are:
//
// [ INCLUDE ( column_name [, ... ] ) ]
// [ WITH ( storage_parameter [= value] [, ... ] ) ]
// [ USING INDEX TABLESPACE tablespace_name ]
//
// exclude_element in an EXCLUDE constraint is:
//
// { column_name | ( expression ) } [ COLLATE collation ] [ opclass [ ( opclass_parameter = value [, ... ] ) ] ] [ ASC | DESC ] [ NULLS { FIRST | LAST } ]
//
// referential_action in a FOREIGN KEY/REFERENCES constraint is:
//
// { NO ACTION | RESTRICT | CASCADE | SET NULL [ ( column_name [, ... ] ) ] | SET DEFAULT [ ( column_name [, ... ] ) ] }
//
// URL: https://www.postgresql.org/docs/18/sql-altertable.html

#[test]
fn alter_table_parses() {
    run_cases(&[
        r#"ALTER TABLE IF EXISTS ONLY my_table ADD COLUMN IF NOT EXISTS new_column integer COLLATE "C" DEFAULT 0"#,
        r#"ALTER TABLE my_table DROP COLUMN IF EXISTS old_column CASCADE"#,
        r#"ALTER TABLE my_table ALTER COLUMN id TYPE bigint USING id::bigint"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET DEFAULT 1"#,
        r#"ALTER TABLE my_table ALTER COLUMN id DROP DEFAULT"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET NOT NULL"#,
        r#"ALTER TABLE my_table ALTER COLUMN id DROP NOT NULL"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET EXPRESSION AS (id + 1)"#,
        r#"ALTER TABLE my_table ALTER COLUMN id DROP EXPRESSION IF EXISTS"#,
        r#"ALTER TABLE my_table ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (START WITH 1)"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET GENERATED BY DEFAULT"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET INCREMENT BY 2"#,
        r#"ALTER TABLE my_table ALTER COLUMN id RESTART WITH 10"#,
        r#"ALTER TABLE my_table ALTER COLUMN id DROP IDENTITY IF EXISTS"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET STATISTICS DEFAULT"#,
        r#"ALTER TABLE my_table ALTER COLUMN id SET (n_distinct = 10)"#,
        r#"ALTER TABLE my_table ALTER COLUMN id RESET (n_distinct)"#,
        r#"ALTER TABLE my_table ALTER COLUMN body SET STORAGE EXTENDED"#,
        r#"ALTER TABLE my_table ALTER COLUMN body SET COMPRESSION pglz"#,
        r#"ALTER TABLE my_table ADD CONSTRAINT nn NOT NULL id NO INHERIT"#,
        r#"ALTER TABLE my_table ADD CONSTRAINT chk CHECK (id > 0) NOT VALID"#,
        r#"ALTER TABLE my_table ADD CONSTRAINT uq UNIQUE NULLS NOT DISTINCT (id) INCLUDE (body) WITH (fillfactor = 80) USING INDEX TABLESPACE fastspace"#,
        r#"ALTER TABLE my_table ADD CONSTRAINT pk PRIMARY KEY USING INDEX my_index"#,
        r#"ALTER TABLE my_table ALTER CONSTRAINT fk DEFERRABLE INITIALLY DEFERRED ENFORCED"#,
        r#"ALTER TABLE my_table ALTER CONSTRAINT nn NO INHERIT"#,
        r#"ALTER TABLE my_table VALIDATE CONSTRAINT chk"#,
        r#"ALTER TABLE my_table DROP CONSTRAINT IF EXISTS chk RESTRICT"#,
        r#"ALTER TABLE my_table DISABLE TRIGGER USER"#,
        r#"ALTER TABLE my_table ENABLE REPLICA TRIGGER trg"#,
        r#"ALTER TABLE my_table DISABLE RULE rewrite_rule"#,
        r#"ALTER TABLE my_table ENABLE ALWAYS RULE rewrite_rule"#,
        r#"ALTER TABLE my_table DISABLE ROW LEVEL SECURITY"#,
        r#"ALTER TABLE my_table ENABLE ROW LEVEL SECURITY"#,
        r#"ALTER TABLE my_table FORCE ROW LEVEL SECURITY"#,
        r#"ALTER TABLE my_table NO FORCE ROW LEVEL SECURITY"#,
        r#"ALTER TABLE my_table CLUSTER ON my_index"#,
        r#"ALTER TABLE my_table SET WITHOUT CLUSTER"#,
        r#"ALTER TABLE my_table SET WITHOUT OIDS"#,
        r#"ALTER TABLE my_table SET ACCESS METHOD DEFAULT"#,
        r#"ALTER TABLE my_table SET TABLESPACE fastspace"#,
        r#"ALTER TABLE my_table SET LOGGED"#,
        r#"ALTER TABLE my_table SET UNLOGGED"#,
        r#"ALTER TABLE my_table SET (fillfactor = 80)"#,
        r#"ALTER TABLE my_table RESET (fillfactor)"#,
        r#"ALTER TABLE my_table INHERIT parent_table"#,
        r#"ALTER TABLE my_table NO INHERIT parent_table"#,
        r#"ALTER TABLE my_table OF my_type"#,
        r#"ALTER TABLE my_table NOT OF"#,
        r#"ALTER TABLE my_table OWNER TO CURRENT_USER"#,
        r#"ALTER TABLE my_table REPLICA IDENTITY DEFAULT"#,
        r#"ALTER TABLE my_table REPLICA IDENTITY USING INDEX my_index"#,
        r#"ALTER TABLE my_table RENAME COLUMN old_column TO new_column"#,
        r#"ALTER TABLE my_table RENAME CONSTRAINT old_constraint TO new_constraint"#,
        r#"ALTER TABLE my_table RENAME TO my_table_new"#,
        r#"ALTER TABLE my_table SET SCHEMA archive"#,
        r#"ALTER TABLE ALL IN TABLESPACE oldspace OWNED BY app_user SET TABLESPACE newspace NOWAIT"#,
        r#"ALTER TABLE parent ATTACH PARTITION child FOR VALUES IN (1, 2)"#,
        r#"ALTER TABLE parent ATTACH PARTITION child FOR VALUES FROM (1, MINVALUE) TO (10, MAXVALUE)"#,
        r#"ALTER TABLE parent ATTACH PARTITION child FOR VALUES WITH (MODULUS 4, REMAINDER 1)"#,
        r#"ALTER TABLE parent ATTACH PARTITION child DEFAULT"#,
        r#"ALTER TABLE parent DETACH PARTITION child CONCURRENTLY"#,
        r#"ALTER TABLE parent DETACH PARTITION child FINALIZE"#,
    ]);
}
