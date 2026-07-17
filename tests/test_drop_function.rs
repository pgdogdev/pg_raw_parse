mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP FUNCTION
// Description: remove a function
// Syntax:
// DROP FUNCTION [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropfunction.html

#[test]
fn drop_function_parses() {
    run_cases(&[
        r#"DROP FUNCTION my_function(integer)"#,
        r#"DROP FUNCTION IF EXISTS my_function(integer), my_function(text) CASCADE"#,
    ]);
}
