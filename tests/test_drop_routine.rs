mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP ROUTINE
// Description: remove a routine
// Syntax:
// DROP ROUTINE [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droproutine.html

#[test]
fn drop_routine_parses() {
    run_cases(&[
        r#"DROP ROUTINE my_function(integer)"#,
        r#"DROP ROUTINE IF EXISTS my_function(integer), my_proc(text) CASCADE"#,
    ]);
}
