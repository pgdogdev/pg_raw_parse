mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER OPERATOR FAMILY
// Description: change the definition of an operator family
// Syntax:
// ALTER OPERATOR FAMILY name USING index_method ADD
//   {  OPERATOR strategy_number operator_name ( op_type, op_type )
//               [ FOR SEARCH | FOR ORDER BY sort_family_name ]
//    | FUNCTION support_number [ ( op_type [ , op_type ] ) ]
//               function_name [ ( argument_type [, ...] ) ]
//   } [, ... ]
//
// ALTER OPERATOR FAMILY name USING index_method DROP
//   {  OPERATOR strategy_number ( op_type [ , op_type ] )
//    | FUNCTION support_number ( op_type [ , op_type ] )
//   } [, ... ]
//
// ALTER OPERATOR FAMILY name USING index_method
//     RENAME TO new_name
//
// ALTER OPERATOR FAMILY name USING index_method
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// ALTER OPERATOR FAMILY name USING index_method
//     SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-alteropfamily.html

#[test]
fn alter_operator_family_parses() {
    run_cases(&[
        r#"ALTER OPERATOR FAMILY my_family USING btree ADD OPERATOR 1 =(integer, integer)"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree DROP OPERATOR 1 (integer, integer)"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree RENAME TO my_family_new"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree OWNER TO CURRENT_USER"#,
        r#"ALTER OPERATOR FAMILY my_family USING btree SET SCHEMA public"#,
    ]);
}
