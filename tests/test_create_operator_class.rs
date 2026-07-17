mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE OPERATOR CLASS
// Description: define a new operator class
// Syntax:
// CREATE OPERATOR CLASS name [ DEFAULT ] FOR TYPE data_type
//   USING index_method [ FAMILY family_name ] AS
//   {  OPERATOR strategy_number operator_name [ ( op_type, op_type ) ] [ FOR SEARCH | FOR ORDER BY sort_family_name ]
//    | FUNCTION support_number [ ( op_type [ , op_type ] ) ] function_name ( argument_type [, ...] )
//    | STORAGE storage_type
//   } [, ... ]
//
// URL: https://www.postgresql.org/docs/18/sql-createopclass.html

#[test]
fn create_operator_class_parses() {
    run_cases(&[
        r#"CREATE OPERATOR CLASS my_opclass DEFAULT FOR TYPE integer USING btree AS OPERATOR 1 <(integer, integer), FUNCTION 1 btint4cmp(integer, integer), STORAGE integer"#,
        r#"CREATE OPERATOR CLASS my_opclass FOR TYPE integer USING hash FAMILY my_family AS OPERATOR 1 =(integer, integer), FUNCTION 1 hashint4(integer)"#,
    ]);
}
