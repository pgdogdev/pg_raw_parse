mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP AGGREGATE
// Description: remove an aggregate function
// Syntax:
// DROP AGGREGATE [ IF EXISTS ] name ( aggregate_signature ) [, ...] [ CASCADE | RESTRICT ]
//
// where aggregate_signature is:
//
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropaggregate.html

#[test]
fn drop_aggregate_parses() {
    run_cases(&[
        r#"DROP AGGREGATE my_sum(integer)"#,
        r#"DROP AGGREGATE IF EXISTS my_sum(integer), my_sum(bigint) CASCADE"#,
    ]);
}
