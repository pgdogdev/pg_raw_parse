mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     COMMENT
// Description: define or change the comment of an object
// Syntax:
// COMMENT ON
// {
//   ACCESS METHOD object_name |
//   AGGREGATE aggregate_name ( aggregate_signature ) |
//   CAST (source_type AS target_type) |
//   COLLATION object_name |
//   COLUMN relation_name.column_name |
//   CONSTRAINT constraint_name ON table_name |
//   CONSTRAINT constraint_name ON DOMAIN domain_name |
//   CONVERSION object_name |
//   DATABASE object_name |
//   DOMAIN object_name |
//   EXTENSION object_name |
//   EVENT TRIGGER object_name |
//   FOREIGN DATA WRAPPER object_name |
//   FOREIGN TABLE object_name |
//   FUNCTION function_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   INDEX object_name |
//   LARGE OBJECT large_object_oid |
//   MATERIALIZED VIEW object_name |
//   OPERATOR operator_name (left_type, right_type) |
//   OPERATOR CLASS object_name USING index_method |
//   OPERATOR FAMILY object_name USING index_method |
//   POLICY policy_name ON table_name |
//   [ PROCEDURAL ] LANGUAGE object_name |
//   PROCEDURE procedure_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   PUBLICATION object_name |
//   ROLE object_name |
//   ROUTINE routine_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   RULE rule_name ON table_name |
//   SCHEMA object_name |
//   SEQUENCE object_name |
//   SERVER object_name |
//   STATISTICS object_name |
//   SUBSCRIPTION object_name |
//   TABLE object_name |
//   TABLESPACE object_name |
//   TEXT SEARCH CONFIGURATION object_name |
//   TEXT SEARCH DICTIONARY object_name |
//   TEXT SEARCH PARSER object_name |
//   TEXT SEARCH TEMPLATE object_name |
//   TRANSFORM FOR type_name LANGUAGE lang_name |
//   TRIGGER trigger_name ON table_name |
//   TYPE object_name |
//   VIEW object_name
// } IS { string_literal | NULL }
//
// where aggregate_signature is:
//
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
//
// URL: https://www.postgresql.org/docs/18/sql-comment.html

#[test]
fn comment_parses() {
    run_cases(&[
        r#"COMMENT ON TABLE my_table IS 'table comment'"#,
        r#"COMMENT ON COLUMN my_table.id IS 'column comment'"#,
        r#"COMMENT ON FUNCTION my_function(integer) IS 'function comment'"#,
        r#"COMMENT ON OPERATOR +(integer, integer) IS 'operator comment'"#,
        r#"COMMENT ON CONSTRAINT my_constraint ON my_table IS 'constraint comment'"#,
        r#"COMMENT ON LARGE OBJECT 12345 IS 'large object comment'"#,
        r#"COMMENT ON TABLE my_table IS NULL"#,
    ]);
}
