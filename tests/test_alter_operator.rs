mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER OPERATOR
// Description: change the definition of an operator
// Syntax:
// ALTER OPERATOR name ( { left_type | NONE } , right_type )
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// ALTER OPERATOR name ( { left_type | NONE } , right_type )
//     SET SCHEMA new_schema
//
// ALTER OPERATOR name ( { left_type | NONE } , right_type )
//     SET ( {  RESTRICT = { res_proc | NONE }
//            | JOIN = { join_proc | NONE }
//            | COMMUTATOR = com_op
//            | NEGATOR = neg_op
//            | HASHES
//            | MERGES
//           } [, ... ] )
//
// URL: https://www.postgresql.org/docs/18/sql-alteroperator.html

#[test]
fn alter_operator_parses() {
    run_cases(&[
        r#"ALTER OPERATOR +(integer, integer) OWNER TO CURRENT_USER"#,
        r#"ALTER OPERATOR +(integer, integer) SET SCHEMA public"#,
        r#"ALTER OPERATOR +(integer, integer) SET (RESTRICT = my_restrict, JOIN = my_join)"#,
    ]);
}
