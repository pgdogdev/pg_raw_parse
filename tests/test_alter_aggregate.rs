mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER AGGREGATE
// Description: change the definition of an aggregate function
// Syntax:
// ALTER AGGREGATE name ( aggregate_signature ) RENAME TO new_name
// ALTER AGGREGATE name ( aggregate_signature )
//                 OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER AGGREGATE name ( aggregate_signature ) SET SCHEMA new_schema
//
// where aggregate_signature is:
//
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
//
// URL: https://www.postgresql.org/docs/18/sql-alteraggregate.html

#[test]
fn alter_aggregate_parses() {
    run_cases(&[
        r#"ALTER AGGREGATE my_sum(integer) RENAME TO my_sum_int"#,
        r#"ALTER AGGREGATE my_sum(integer) OWNER TO CURRENT_USER"#,
        r#"ALTER AGGREGATE my_sum(integer) SET SCHEMA analytics"#,
    ]);
}
