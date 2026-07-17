mod common;

use common::run_parse_walk_cases as run_cases;

#[test]
fn select_clause_variations() {
    run_cases(&[
        "SELECT",
        "SELECT ALL",
        "SELECT ALL 1",
        "SELECT DISTINCT id FROM users",
        "SELECT DISTINCT ON (account_id, created_at) account_id, created_at FROM events",
        "SELECT *",
        "SELECT id AS user_id, name display_name, 1 + 2",
        "SELECT FROM users",
        "SELECT id FROM users WHERE active AND age >= 18",
        "SELECT department, count(*) FROM employees GROUP BY department HAVING count(*) > 1",
        "SELECT sum(salary) OVER w FROM employees WINDOW w AS (PARTITION BY department ORDER BY hired_at)",
        "SELECT sum(salary) OVER w, avg(salary) OVER v FROM employees WINDOW w AS (PARTITION BY department), v AS (ORDER BY hired_at)",
        "SELECT id FROM users ORDER BY created_at ASC NULLS LAST, id DESC NULLS FIRST",
        "SELECT id FROM users ORDER BY created_at NULLS FIRST",
        "SELECT id FROM users ORDER BY name USING < NULLS FIRST",
        "SELECT id FROM users LIMIT 10",
        "SELECT id FROM users LIMIT ALL",
        "SELECT id FROM users OFFSET 5",
        "SELECT id FROM users OFFSET 5 ROW",
        "SELECT id FROM users OFFSET 5 ROWS",
        "SELECT id FROM users FETCH FIRST ROW ONLY",
        "SELECT id FROM users FETCH FIRST 10 ROWS ONLY",
        "SELECT id FROM users FETCH NEXT 10 ROW ONLY",
        "SELECT id FROM users FETCH NEXT ROWS ONLY",
        "SELECT id FROM users ORDER BY id FETCH NEXT 10 ROWS WITH TIES",
        "SELECT id FROM users ORDER BY id FETCH NEXT ROW WITH TIES",
        "SELECT id FROM users WHERE active GROUP BY id HAVING count(*) > 0 WINDOW w AS (ORDER BY id) ORDER BY id LIMIT 5 OFFSET 1 ROWS",
    ]);
}

#[test]
fn with_clause_variations() {
    run_cases(&[
        "WITH w AS (SELECT 1 AS id) SELECT * FROM w",
        "WITH w(id, name) AS (SELECT 1, 'a') SELECT id, name FROM w",
        "WITH w AS MATERIALIZED (SELECT 1 AS id) SELECT * FROM w",
        "WITH w AS NOT MATERIALIZED (SELECT 1 AS id) SELECT * FROM w",
        "WITH w AS (VALUES (1), (2)) SELECT * FROM w",
        "WITH w AS (INSERT INTO users (id) VALUES (1) RETURNING id) SELECT * FROM w",
        "WITH w AS (UPDATE users SET active = true RETURNING id) SELECT * FROM w",
        "WITH w AS (DELETE FROM users WHERE active = false RETURNING id) SELECT * FROM w",
        "WITH w AS (MERGE INTO users u USING incoming i ON u.id = i.id WHEN MATCHED THEN UPDATE SET active = true RETURNING u.id) SELECT * FROM w",
        "WITH RECURSIVE t(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM t WHERE n < 3) SELECT * FROM t",
        "WITH RECURSIVE t(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM t WHERE n < 3) SEARCH BREADTH FIRST BY n SET ordercol SELECT * FROM t",
        "WITH RECURSIVE t(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM t WHERE n < 3) SEARCH DEPTH FIRST BY n SET ordercol SELECT * FROM t",
        "WITH RECURSIVE t(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM t WHERE n < 3) CYCLE n SET is_cycle USING path SELECT * FROM t",
        "WITH RECURSIVE t(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM t WHERE n < 3) CYCLE n SET is_cycle TO true DEFAULT false USING path SELECT * FROM t",
        "WITH RECURSIVE t(n, m) AS (SELECT 1, 2 UNION ALL SELECT n + 1, m + 1 FROM t WHERE n < 3) SEARCH BREADTH FIRST BY n, m SET ordercol SELECT * FROM t",
        "WITH RECURSIVE t(n, m) AS (SELECT 1, 2 UNION ALL SELECT n + 1, m + 1 FROM t WHERE n < 3) CYCLE n, m SET is_cycle USING path SELECT * FROM t",
        "WITH one AS (SELECT 1), two AS (SELECT 2) SELECT * FROM one, two",
    ]);
}

#[test]
fn from_item_variations() {
    run_cases(&[
        "SELECT * FROM users",
        "SELECT * FROM ONLY users",
        "SELECT * FROM users *",
        "SELECT * FROM users AS u",
        "SELECT * FROM users u",
        "SELECT * FROM users AS u (id, name)",
        "SELECT * FROM users TABLESAMPLE bernoulli (10)",
        "SELECT * FROM users AS u TABLESAMPLE system (25) REPEATABLE (42)",
        "SELECT * FROM (SELECT 1 AS id) s",
        "SELECT * FROM LATERAL (SELECT 1 AS id) AS s(id)",
        "SELECT * FROM LATERAL (SELECT 1 AS id) s",
        "WITH w AS (SELECT 1 AS id) SELECT * FROM w AS cte(id)",
        "SELECT * FROM foo()",
        "SELECT * FROM generate_series(1, 3)",
        "SELECT * FROM generate_series(1, 3) WITH ORDINALITY",
        "SELECT * FROM generate_series(1, 3) g(value)",
        "SELECT * FROM LATERAL generate_series(1, 3) WITH ORDINALITY AS g(value, ordinality)",
        "SELECT * FROM json_to_record('{\"a\":1}') AS x(a int)",
        "SELECT * FROM json_to_record('{\"a\":1}') x(a int)",
        "SELECT * FROM LATERAL json_to_record('{\"a\":1}') AS (a int)",
        "SELECT * FROM ROWS FROM (generate_series(1, 3), generate_series(4, 6))",
        "SELECT * FROM ROWS FROM (json_to_record('{\"a\":1}') AS (a int))",
        "SELECT * FROM LATERAL ROWS FROM (generate_series(1, 3)) AS r(n)",
        "SELECT * FROM ROWS FROM (json_to_record('{\"a\":1}') AS (a int), generate_series(1, 3)) WITH ORDINALITY AS r(a, b, ordinality)",
        "SELECT * FROM users JOIN orders ON users.id = orders.user_id",
        "SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id",
        "SELECT * FROM users LEFT JOIN orders ON users.id = orders.user_id",
        "SELECT * FROM users LEFT OUTER JOIN orders ON users.id = orders.user_id",
        "SELECT * FROM users RIGHT JOIN orders USING (user_id)",
        "SELECT * FROM users RIGHT OUTER JOIN orders USING (user_id)",
        "SELECT * FROM users FULL OUTER JOIN orders USING (user_id)",
        "SELECT * FROM users FULL JOIN orders USING (user_id) AS joined_cols",
        "SELECT * FROM users JOIN orders USING (user_id, tenant_id)",
        "SELECT * FROM users NATURAL INNER JOIN orders",
        "SELECT * FROM users NATURAL LEFT JOIN orders",
        "SELECT * FROM users NATURAL FULL OUTER JOIN orders",
        "SELECT * FROM users CROSS JOIN orders",
        "SELECT * FROM users, orders, LATERAL generate_series(1, users.id) AS g(n)",
    ]);
}

#[test]
fn grouping_element_variations() {
    run_cases(&[
        "SELECT count(*) FROM users GROUP BY ()",
        "SELECT department, count(*) FROM employees GROUP BY department",
        "SELECT department, role, count(*) FROM employees GROUP BY (department, role)",
        "SELECT department, role, count(*) FROM employees GROUP BY ALL department, role",
        "SELECT department, role, count(*) FROM employees GROUP BY DISTINCT department, role",
        "SELECT department, role, count(*) FROM employees GROUP BY ROLLUP (department, (role, location))",
        "SELECT department, role, count(*) FROM employees GROUP BY CUBE (department, (role, location))",
        "SELECT department, role, count(*) FROM employees GROUP BY GROUPING SETS ((), department, (department, role), ROLLUP (location, role), CUBE (department, location))",
    ]);
}

#[test]
fn set_operation_variations() {
    run_cases(&[
        "SELECT 1 UNION SELECT 2",
        "SELECT 1 UNION ALL SELECT 2",
        "SELECT 1 UNION DISTINCT SELECT 2",
        "SELECT 1 INTERSECT SELECT 1",
        "SELECT 1 INTERSECT ALL SELECT 1",
        "SELECT 1 INTERSECT DISTINCT SELECT 1",
        "SELECT 1 EXCEPT SELECT 2",
        "SELECT 1 EXCEPT ALL SELECT 2",
        "SELECT 1 EXCEPT DISTINCT SELECT 2",
        "(SELECT 1 UNION SELECT 2) EXCEPT SELECT 3 ORDER BY 1 LIMIT 1",
    ]);
}

#[test]
fn locking_clause_variations() {
    run_cases(&[
        "SELECT * FROM users FOR UPDATE",
        "SELECT * FROM users FOR NO KEY UPDATE",
        "SELECT * FROM users FOR SHARE",
        "SELECT * FROM users FOR KEY SHARE",
        "SELECT * FROM users FOR UPDATE OF users",
        "SELECT * FROM users u JOIN orders o ON u.id = o.user_id FOR UPDATE OF u, o",
        "SELECT * FROM users FOR UPDATE NOWAIT",
        "SELECT * FROM users FOR UPDATE SKIP LOCKED",
        "SELECT * FROM users FOR UPDATE NOWAIT FOR SHARE SKIP LOCKED",
    ]);
}

#[test]
fn table_statement_variations() {
    run_cases(&["TABLE users", "TABLE ONLY users", "TABLE users *"]);
}

#[test]
fn parameter_variations() {
    run_cases(&[
        "SELECT $1",
        "SELECT $1::int AS id, coalesce($2, 'fallback') name",
        "SELECT DISTINCT ON ($1) $1, id FROM users",
        "SELECT * FROM users WHERE id = $1 AND active = $2",
        "SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > $1)",
        "SELECT department, count(*) FROM employees GROUP BY department HAVING count(*) > $1",
        "SELECT count(*) FROM employees GROUP BY ROLLUP (department, $1)",
        "SELECT sum(salary) OVER w FROM employees WINDOW w AS (PARTITION BY $1 ORDER BY hired_at)",
        "SELECT * FROM users TABLESAMPLE bernoulli ($1) REPEATABLE ($2)",
        "SELECT * FROM generate_series($1, $2)",
        "SELECT * FROM ROWS FROM (generate_series($1, $2), generate_series($3, $4))",
        "SELECT * FROM users JOIN orders ON users.id = orders.user_id AND orders.total > $1",
        "WITH w AS (SELECT $1 AS id) SELECT * FROM w WHERE id = $2",
        "WITH w AS (VALUES ($1), ($2)) SELECT * FROM w",
        "WITH w AS (INSERT INTO users (id) VALUES ($1) RETURNING id) SELECT * FROM w",
        "SELECT $1 UNION ALL SELECT $2",
        "SELECT $1 INTERSECT SELECT $2",
        "SELECT $1 EXCEPT SELECT $2",
        "SELECT id FROM users ORDER BY coalesce($1, id) ASC NULLS LAST",
        "SELECT id FROM users LIMIT $1",
        "SELECT id FROM users OFFSET $1 ROWS",
        "SELECT id FROM users FETCH FIRST $1 ROWS ONLY",
        "SELECT id FROM users ORDER BY id FETCH NEXT $1 ROWS WITH TIES",
    ]);
}

// Command:     SELECT
// Description: retrieve rows from a table or view
// Syntax:
// [ WITH [ RECURSIVE ] with_query [, ...] ]
// SELECT [ ALL | DISTINCT [ ON ( expression [, ...] ) ] ]
//     [ { * | expression [ [ AS ] output_name ] } [, ...] ]
//     [ FROM from_item [, ...] ]
//     [ WHERE condition ]
//     [ GROUP BY [ ALL | DISTINCT ] grouping_element [, ...] ]
//     [ HAVING condition ]
//     [ WINDOW window_name AS ( window_definition ) [, ...] ]
//     [ { UNION | INTERSECT | EXCEPT } [ ALL | DISTINCT ] select ]
//     [ ORDER BY expression [ ASC | DESC | USING operator ] [ NULLS { FIRST | LAST } ] [, ...] ]
//     [ LIMIT { count | ALL } ]
//     [ OFFSET start [ ROW | ROWS ] ]
//     [ FETCH { FIRST | NEXT } [ count ] { ROW | ROWS } { ONLY | WITH TIES } ]
//     [ FOR { UPDATE | NO KEY UPDATE | SHARE | KEY SHARE } [ OF from_reference [, ...] ] [ NOWAIT | SKIP LOCKED ] [...] ]

// where from_item can be one of:

//     [ ONLY ] table_name [ * ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//                 [ TABLESAMPLE sampling_method ( argument [, ...] ) [ REPEATABLE ( seed ) ] ]
//     [ LATERAL ] ( select ) [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//     with_query_name [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//     [ LATERAL ] function_name ( [ argument [, ...] ] )
//                 [ WITH ORDINALITY ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//     [ LATERAL ] function_name ( [ argument [, ...] ] ) [ AS ] alias ( column_definition [, ...] )
//     [ LATERAL ] function_name ( [ argument [, ...] ] ) AS ( column_definition [, ...] )
//     [ LATERAL ] ROWS FROM( function_name ( [ argument [, ...] ] ) [ AS ( column_definition [, ...] ) ] [, ...] )
//                 [ WITH ORDINALITY ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//     from_item join_type from_item { ON join_condition | USING ( join_column [, ...] ) [ AS join_using_alias ] }
//     from_item NATURAL join_type from_item
//     from_item CROSS JOIN from_item

// and grouping_element can be one of:

//     ( )
//     expression
//     ( expression [, ...] )
//     ROLLUP ( { expression | ( expression [, ...] ) } [, ...] )
//     CUBE ( { expression | ( expression [, ...] ) } [, ...] )
//     GROUPING SETS ( grouping_element [, ...] )

// and with_query is:

//     with_query_name [ ( column_name [, ...] ) ] AS [ [ NOT ] MATERIALIZED ] ( select | values | insert | update | delete | merge )
//         [ SEARCH { BREADTH | DEPTH } FIRST BY column_name [, ...] SET search_seq_col_name ]
//         [ CYCLE column_name [, ...] SET cycle_mark_col_name [ TO cycle_mark_value DEFAULT cycle_mark_default ] USING cycle_path_col_name ]

// TABLE [ ONLY ] table_name [ * ]
//
// URL: https://www.postgresql.org/docs/18/sql-select.html
