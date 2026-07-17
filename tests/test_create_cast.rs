mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_cast_parses() {
    run_cases(&[
        r#"CREATE CAST (integer AS bigint) WITHOUT FUNCTION AS IMPLICIT"#,
        r#"CREATE CAST (integer AS text) WITH FUNCTION int4out(integer) AS ASSIGNMENT"#,
        r#"CREATE CAST (my_type AS text) WITH INOUT"#,
    ]);
}
