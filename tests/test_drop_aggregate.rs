mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_aggregate_parses() {
    run_cases(&[
        r#"DROP AGGREGATE my_sum(integer)"#,
        r#"DROP AGGREGATE IF EXISTS my_sum(integer), my_sum(bigint) CASCADE"#,
    ]);
}
