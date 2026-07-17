mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     SECURITY LABEL
// Description: define or change a security label applied to an object
// Syntax:
// SECURITY LABEL [ FOR provider ] ON
// {
//   TABLE object_name |
//   COLUMN table_name.column_name |
//   AGGREGATE aggregate_name ( aggregate_signature ) |
//   DATABASE object_name |
//   DOMAIN object_name |
//   EVENT TRIGGER object_name |
//   FOREIGN TABLE object_name |
//   FUNCTION function_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   LARGE OBJECT large_object_oid |
//   MATERIALIZED VIEW object_name |
//   [ PROCEDURAL ] LANGUAGE object_name |
//   PROCEDURE procedure_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   PUBLICATION object_name |
//   ROLE object_name |
//   ROUTINE routine_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   SCHEMA object_name |
//   SEQUENCE object_name |
//   SUBSCRIPTION object_name |
//   TABLESPACE object_name |
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
// URL: https://www.postgresql.org/docs/18/sql-security-label.html

#[test]
fn security_label_parses() {
    run_cases(&[
        r#"SECURITY LABEL ON TABLE my_table IS 'classified'"#,
        r#"SECURITY LABEL FOR selinux ON COLUMN my_table.id IS 'system_u:object_r:sepgsql_table_t:s0'"#,
        r#"SECURITY LABEL ON FUNCTION my_function(integer) IS NULL"#,
    ]);
}
