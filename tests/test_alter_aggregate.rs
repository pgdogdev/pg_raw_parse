mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_aggregate_parses() {
    run_cases(&[
        r#"ALTER AGGREGATE my_sum(integer) RENAME TO my_sum_int"#,
        r#"ALTER AGGREGATE my_sum(integer) OWNER TO CURRENT_USER"#,
        r#"ALTER AGGREGATE my_sum(integer) SET SCHEMA analytics"#,
    ]);
}
