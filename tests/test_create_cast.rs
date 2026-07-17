mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE CAST
// Description: define a new cast
// Syntax:
// CREATE CAST (source_type AS target_type)
//     WITH FUNCTION function_name [ (argument_type [, ...]) ]
//     [ AS ASSIGNMENT | AS IMPLICIT ]
//
// CREATE CAST (source_type AS target_type)
//     WITHOUT FUNCTION
//     [ AS ASSIGNMENT | AS IMPLICIT ]
//
// CREATE CAST (source_type AS target_type)
//     WITH INOUT
//     [ AS ASSIGNMENT | AS IMPLICIT ]
//
// URL: https://www.postgresql.org/docs/18/sql-createcast.html

#[test]
fn create_cast_parses() {
    run_cases(&[
        r#"CREATE CAST (integer AS bigint) WITHOUT FUNCTION AS IMPLICIT"#,
        r#"CREATE CAST (integer AS text) WITH FUNCTION int4out(integer) AS ASSIGNMENT"#,
        r#"CREATE CAST (my_type AS text) WITH INOUT"#,
    ]);
}
