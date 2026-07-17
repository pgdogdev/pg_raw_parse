mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE AGGREGATE
// Description: define a new aggregate function
// Syntax:
// CREATE [ OR REPLACE ] AGGREGATE name ( [ argmode ] [ argname ] arg_data_type [ , ... ] ) (
//     SFUNC = sfunc,
//     STYPE = state_data_type
//     [ , SSPACE = state_data_size ]
//     [ , FINALFUNC = ffunc ]
//     [ , FINALFUNC_EXTRA ]
//     [ , FINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , COMBINEFUNC = combinefunc ]
//     [ , SERIALFUNC = serialfunc ]
//     [ , DESERIALFUNC = deserialfunc ]
//     [ , INITCOND = initial_condition ]
//     [ , MSFUNC = msfunc ]
//     [ , MINVFUNC = minvfunc ]
//     [ , MSTYPE = mstate_data_type ]
//     [ , MSSPACE = mstate_data_size ]
//     [ , MFINALFUNC = mffunc ]
//     [ , MFINALFUNC_EXTRA ]
//     [ , MFINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , MINITCOND = minitial_condition ]
//     [ , SORTOP = sort_operator ]
//     [ , PARALLEL = { SAFE | RESTRICTED | UNSAFE } ]
// )
//
// CREATE [ OR REPLACE ] AGGREGATE name ( [ [ argmode ] [ argname ] arg_data_type [ , ... ] ]
//                         ORDER BY [ argmode ] [ argname ] arg_data_type [ , ... ] ) (
//     SFUNC = sfunc,
//     STYPE = state_data_type
//     [ , SSPACE = state_data_size ]
//     [ , FINALFUNC = ffunc ]
//     [ , FINALFUNC_EXTRA ]
//     [ , FINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , INITCOND = initial_condition ]
//     [ , PARALLEL = { SAFE | RESTRICTED | UNSAFE } ]
//     [ , HYPOTHETICAL ]
// )
//
// or the old syntax
//
// CREATE [ OR REPLACE ] AGGREGATE name (
//     BASETYPE = base_type,
//     SFUNC = sfunc,
//     STYPE = state_data_type
//     [ , SSPACE = state_data_size ]
//     [ , FINALFUNC = ffunc ]
//     [ , FINALFUNC_EXTRA ]
//     [ , FINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , COMBINEFUNC = combinefunc ]
//     [ , SERIALFUNC = serialfunc ]
//     [ , DESERIALFUNC = deserialfunc ]
//     [ , INITCOND = initial_condition ]
//     [ , MSFUNC = msfunc ]
//     [ , MINVFUNC = minvfunc ]
//     [ , MSTYPE = mstate_data_type ]
//     [ , MSSPACE = mstate_data_size ]
//     [ , MFINALFUNC = mffunc ]
//     [ , MFINALFUNC_EXTRA ]
//     [ , MFINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , MINITCOND = minitial_condition ]
//     [ , SORTOP = sort_operator ]
// )
//
// URL: https://www.postgresql.org/docs/18/sql-createaggregate.html

#[test]
fn create_aggregate_parses() {
    run_cases(&[
        r#"CREATE AGGREGATE my_sum(integer) (SFUNC = int4pl, STYPE = integer)"#,
        r#"CREATE AGGREGATE my_sum(*) (SFUNC = int8inc, STYPE = bigint)"#,
        r#"CREATE AGGREGATE my_sum(ORDER BY integer) (SFUNC = int4pl, STYPE = integer)"#,
        r#"CREATE AGGREGATE my_hypothetical(integer ORDER BY integer) (SFUNC = int4pl, STYPE = integer, FINALFUNC = int4abs)"#,
    ]);
}
