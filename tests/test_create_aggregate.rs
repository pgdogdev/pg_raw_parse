mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_aggregate_parses() {
    run_cases(&[
        r#"CREATE AGGREGATE my_sum(integer) (SFUNC = int4pl, STYPE = integer)"#,
        r#"CREATE AGGREGATE my_sum(*) (SFUNC = int8inc, STYPE = bigint)"#,
        r#"CREATE AGGREGATE my_sum(ORDER BY integer) (SFUNC = int4pl, STYPE = integer)"#,
        r#"CREATE AGGREGATE my_hypothetical(integer ORDER BY integer) (SFUNC = int4pl, STYPE = integer, FINALFUNC = int4abs)"#,
    ]);
}
