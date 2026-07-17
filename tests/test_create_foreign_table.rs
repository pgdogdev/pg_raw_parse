mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE FOREIGN TABLE
// Description: define a new foreign table
// Syntax:
// CREATE FOREIGN TABLE [ IF NOT EXISTS ] table_name ( [
//   { column_name data_type [ OPTIONS ( option 'value' [, ... ] ) ] [ COLLATE collation ] [ column_constraint [ ... ] ]
//     | table_constraint
//     | LIKE source_table [ like_option ... ] }
//     [, ... ]
// ] )
// [ INHERITS ( parent_table [, ... ] ) ]
//   SERVER server_name
// [ OPTIONS ( option 'value' [, ... ] ) ]
//
// CREATE FOREIGN TABLE [ IF NOT EXISTS ] table_name
//   PARTITION OF parent_table [ (
//   { column_name [ WITH OPTIONS ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ) ]
// { FOR VALUES partition_bound_spec | DEFAULT }
//   SERVER server_name
// [ OPTIONS ( option 'value' [, ... ] ) ]
//
// where column_constraint is:
//
// [ CONSTRAINT constraint_name ]
// { NOT NULL [ NO INHERIT ] |
//   NULL |
//   CHECK ( expression ) [ NO INHERIT ] |
//   DEFAULT default_expr |
//   GENERATED ALWAYS AS ( generation_expr ) [ STORED | VIRTUAL ] }
// [ ENFORCED | NOT ENFORCED ]
//
// and table_constraint is:
//
// [ CONSTRAINT constraint_name ]
// {  NOT NULL column_name [ NO INHERIT ] |
//    CHECK ( expression ) [ NO INHERIT ] }
// [ ENFORCED | NOT ENFORCED ]
//
// and like_option is:
//
// { INCLUDING | EXCLUDING } { COMMENTS | CONSTRAINTS | DEFAULTS | GENERATED | STATISTICS | ALL }
//
// and partition_bound_spec is:
//
// IN ( partition_bound_expr [, ...] ) |
// FROM ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] )
//   TO ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] ) |
// WITH ( MODULUS numeric_literal, REMAINDER numeric_literal )
//
// URL: https://www.postgresql.org/docs/18/sql-createforeigntable.html

#[test]
fn create_foreign_table_parses() {
    run_cases(&[
        r#"CREATE FOREIGN TABLE ft (id integer OPTIONS (column_name 'id'), body text COLLATE "C" NOT NULL) SERVER my_server OPTIONS (schema_name 'public', table_name 'remote')"#,
        r#"CREATE FOREIGN TABLE IF NOT EXISTS ft PARTITION OF parent FOR VALUES IN (1) SERVER my_server"#,
        r#"CREATE FOREIGN TABLE ft (LIKE source INCLUDING DEFAULTS, CHECK (id > 0), FOREIGN KEY (id) REFERENCES other(id)) SERVER my_server"#,
    ]);
}
