mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP PROCEDURE
// Description: remove a procedure
// Syntax:
// DROP PROCEDURE [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropprocedure.html

#[test]
fn drop_procedure_parses() {
    run_cases(&[
        r#"DROP PROCEDURE my_proc(integer)"#,
        r#"DROP PROCEDURE IF EXISTS my_proc(integer), my_proc(text) CASCADE"#,
    ]);
}
