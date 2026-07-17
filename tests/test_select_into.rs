mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     SELECT INTO
// Description: define a new table from the results of a query
// Syntax:
// [ WITH [ RECURSIVE ] with_query [, ...] ]
// SELECT [ ALL | DISTINCT [ ON ( expression [, ...] ) ] ]
//     [ { * | expression [ [ AS ] output_name ] } [, ...] ]
//     INTO [ TEMPORARY | TEMP | UNLOGGED ] [ TABLE ] new_table
//     [ FROM from_item [, ...] ]
//     [ WHERE condition ]
//     [ GROUP BY expression [, ...] ]
//     [ HAVING condition ]
//     [ WINDOW window_name AS ( window_definition ) [, ...] ]
//     [ { UNION | INTERSECT | EXCEPT } [ ALL | DISTINCT ] select ]
//     [ ORDER BY expression [ ASC | DESC | USING operator ] [ NULLS { FIRST | LAST } ] [, ...] ]
//     [ LIMIT { count | ALL } ]
//     [ OFFSET start [ ROW | ROWS ] ]
//     [ FETCH { FIRST | NEXT } [ count ] { ROW | ROWS } ONLY ]
//     [ FOR { UPDATE | SHARE } [ OF table_name [, ...] ] [ NOWAIT ] [...] ]
//
// URL: https://www.postgresql.org/docs/18/sql-selectinto.html

#[test]
fn select_into_parses() {
    run_cases(&[
        r#"SELECT 1 AS id INTO new_table"#,
        r#"SELECT 1 AS id INTO TEMP TABLE new_table"#,
        r#"SELECT 1 AS id INTO UNLOGGED TABLE new_table FROM source_table WHERE id > 0"#,
    ]);
}
