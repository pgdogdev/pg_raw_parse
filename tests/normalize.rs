use pg_raw_parse::{deparse, normalize::normalize, parse};

fn assert_normalized(query: &str, expected: &str) {
    let original = parse(query).expect("valid input SQL");
    let stmt = original.first().expect("one input statement");
    let before = deparse(stmt).expect("original SQL deparses");
    let expected = parse(expected).expect("valid expected SQL");
    let expected =
        deparse(expected.first().expect("one expected statement")).expect("expected SQL deparses");

    let normalized = normalize(stmt);
    let actual = deparse(&*normalized).expect("normalized SQL deparses");
    assert_eq!(actual.as_str(), expected.as_str(), "{query}");
    parse(actual.as_str()).expect("normalized SQL reparses");
    assert_eq!(
        deparse(stmt).expect("original SQL still deparses").as_str(),
        before.as_str(),
        "normalizing must not mutate the original AST"
    );
}

#[test]
fn type_modifiers_remain_literals() {
    for (query, expected) in [
        (
            "SELECT 'abc'::varchar(10), 42::numeric(10, 2)",
            "SELECT $1::varchar(10), $2::numeric(10, 2)",
        ),
        (
            "SELECT 'a'::char, B'101'::bit(3)",
            "SELECT $1::char, $2::bit(3)",
        ),
        (
            "SELECT '2026-09-14'::timestamp(3) with time zone",
            "SELECT $1::timestamp(3) with time zone",
        ),
        (
            "SELECT '12:00'::time(2) with time zone",
            "SELECT $1::time(2) with time zone",
        ),
        (
            "SELECT INTERVAL '1.234' SECOND(2), 42",
            "SELECT $1::interval SECOND(2), $2",
        ),
        (
            "SELECT INTERVAL '1-2' YEAR TO MONTH",
            "SELECT $1::interval YEAR TO MONTH",
        ),
        (
            "CREATE TABLE t (a varchar(20), b numeric(10, 2), c interval DAY TO SECOND(3))",
            "CREATE TABLE t (a varchar(20), b numeric(10, 2), c interval DAY TO SECOND(3))",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn json_table_paths_remain_strings() {
    for (query, expected) in [
        (
            "SELECT * FROM JSON_TABLE('[1,2]'::jsonb, '$[*]' COLUMNS (value int PATH '$')) AS jt",
            "SELECT * FROM JSON_TABLE($1::jsonb, '$[*]' COLUMNS (value int PATH '$')) AS jt",
        ),
        (
            "SELECT * FROM JSON_TABLE('{\"a\":[1]}'::jsonb, '$' AS root COLUMNS (NESTED PATH '$.a[*]' AS nested COLUMNS (value int PATH '$' DEFAULT 0 ON EMPTY))) AS jt",
            "SELECT * FROM JSON_TABLE($1::jsonb, '$' AS root COLUMNS (NESTED PATH '$.a[*]' AS nested COLUMNS (value int PATH '$' DEFAULT $2 ON EMPTY))) AS jt",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn xmlroot_options_remain_constants() {
    for (query, expected) in [
        (
            "SELECT XMLROOT('<a/>'::xml, VERSION '1.0', STANDALONE YES), 42",
            "SELECT XMLROOT($1::xml, VERSION '1.0', STANDALONE YES), $2",
        ),
        (
            "SELECT XMLROOT('<a/>'::xml, VERSION NO VALUE, STANDALONE NO)",
            "SELECT XMLROOT($1::xml, VERSION NO VALUE, STANDALONE NO)",
        ),
        (
            "SELECT XMLROOT('<a/>'::xml, VERSION '1.0', STANDALONE NO VALUE)",
            "SELECT XMLROOT($1::xml, VERSION '1.0', STANDALONE NO VALUE)",
        ),
        (
            "SELECT XMLROOT(XMLCONCAT('<a/>'::xml, '<b/>'::xml), VERSION NO VALUE)",
            "SELECT XMLROOT(XMLCONCAT($1::xml, $2::xml), VERSION NO VALUE)",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn unicode_normalization_forms_remain_keywords() {
    for (query, expected) in [
        (
            "SELECT NORMALIZE('hello', NFC), 42",
            "SELECT NORMALIZE($1, NFC), $2",
        ),
        (
            "SELECT NORMALIZE('hello', NFD), NORMALIZE('world', NFKC)",
            "SELECT NORMALIZE($1, NFD), NORMALIZE($2, NFKC)",
        ),
        (
            "SELECT 'hello' IS NFKD NORMALIZED",
            "SELECT $1 IS NFKD NORMALIZED",
        ),
        (
            "SELECT 'hello' IS NOT NFC NORMALIZED",
            "SELECT $1 IS NOT NFC NORMALIZED",
        ),
        ("SELECT NORMALIZE('hello')", "SELECT NORMALIZE($1)"),
        (
            "SELECT pg_catalog.normalize('hello', 'NFC')",
            "SELECT pg_catalog.normalize($1, $2)",
        ),
        (
            "SELECT pg_catalog.is_normalized('hello', 'NFC')",
            "SELECT pg_catalog.is_normalized($1, $2)",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn cycle_mark_values_remain_constants() {
    for (query, expected) in [
        (
            "WITH RECURSIVE t(id) AS (SELECT 1 UNION ALL SELECT id + 1 FROM t) CYCLE id SET cycle USING path SELECT * FROM t",
            "WITH RECURSIVE t(id) AS (SELECT $1 UNION ALL SELECT id + $2 FROM t) CYCLE id SET cycle USING path SELECT * FROM t",
        ),
        (
            "WITH RECURSIVE t(id) AS (SELECT 1) CYCLE id SET cycle TO 'yes' DEFAULT 'no' USING path SELECT 42 FROM t",
            "WITH RECURSIVE t(id) AS (SELECT $2) CYCLE id SET cycle TO 'yes' DEFAULT 'no' USING path SELECT $1 FROM t",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn utility_statements_still_normalize_expressions() {
    for (query, expected) in [
        (
            "EXPLAIN (ANALYZE FALSE, COSTS FALSE) SELECT 42",
            "EXPLAIN (ANALYZE FALSE, COSTS FALSE) SELECT $1",
        ),
        (
            "CREATE VIEW v AS SELECT 'abc'::varchar(10)",
            "CREATE VIEW v AS SELECT $1::varchar(10)",
        ),
        (
            "COPY (SELECT 42) TO STDOUT WITH (FORMAT csv, DELIMITER ',')",
            "COPY (SELECT $1) TO STDOUT WITH (FORMAT csv, DELIMITER ',')",
        ),
        (
            "CREATE TABLE t (id int DEFAULT 42 CHECK (id > 0))",
            "CREATE TABLE t (id int DEFAULT $1 CHECK (id > $2))",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn group_and_order_by_ordinals_remain_integers() {
    for (query, expected) in [
        (
            "SELECT a, sum(b) FROM t WHERE c = 42 GROUP BY 1 ORDER BY 1 LIMIT 10",
            "SELECT a, sum(b) FROM t WHERE c = $1 GROUP BY 1 ORDER BY 1 LIMIT $2",
        ),
        (
            "SELECT 42, $7 FROM t GROUP BY 1, 2 ORDER BY 2 DESC NULLS LAST, 1",
            "SELECT $1, $2 FROM t GROUP BY 1, 2 ORDER BY 2 DESC NULLS LAST, 1",
        ),
        (
            "SELECT 42 GROUP BY (1) ORDER BY (1)",
            "SELECT $1 GROUP BY (1) ORDER BY (1)",
        ),
        (
            "SELECT 42 GROUP BY 0 ORDER BY -1",
            "SELECT $1 GROUP BY 0 ORDER BY -1",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn grouping_sets_and_nested_query_ordinals_remain_integers() {
    for (query, expected) in [
        (
            "SELECT a, b FROM t WHERE c = 42 GROUP BY GROUPING SETS ((1, 2), ROLLUP(1, 2), CUBE(1), ()) ORDER BY 2",
            "SELECT a, b FROM t WHERE c = $1 GROUP BY GROUPING SETS ((1, 2), ROLLUP(1, 2), CUBE(1), ()) ORDER BY 2",
        ),
        (
            "SELECT (SELECT 5 ORDER BY 1), 6 ORDER BY 2",
            "SELECT (SELECT $1 ORDER BY 1), $2 ORDER BY 2",
        ),
        (
            "WITH t AS (SELECT 42 AS a GROUP BY 1 ORDER BY 1) SELECT * FROM t ORDER BY 1",
            "WITH t AS (SELECT $1 AS a GROUP BY 1 ORDER BY 1) SELECT * FROM t ORDER BY 1",
        ),
        (
            "EXPLAIN SELECT 42 GROUP BY 1 ORDER BY 1",
            "EXPLAIN SELECT $1 GROUP BY 1 ORDER BY 1",
        ),
        (
            "SELECT 1 UNION ALL SELECT 2 ORDER BY 1",
            "SELECT $1 UNION ALL SELECT $2 ORDER BY 1",
        ),
    ] {
        assert_normalized(query, expected);
    }
}

#[test]
fn numeric_expressions_are_not_ordinals() {
    for (query, expected) in [
        (
            "SELECT a FROM t GROUP BY a + 1 ORDER BY a + 2 LIMIT 3",
            "SELECT a FROM t GROUP BY a + $1 ORDER BY a + $2 LIMIT $3",
        ),
        (
            "SELECT a FROM t GROUP BY ROW(1, 2) ORDER BY (3, 4)",
            "SELECT a FROM t GROUP BY ROW($1, $2) ORDER BY ($3, $4)",
        ),
        (
            "SELECT sum(a ORDER BY 1), row_number() OVER (ORDER BY 2) FROM t ORDER BY 1",
            "SELECT sum(a ORDER BY $1), row_number() OVER (ORDER BY $2) FROM t ORDER BY 1",
        ),
        (
            "SELECT percentile_cont(0.5) WITHIN GROUP (ORDER BY 1) FROM t ORDER BY 1",
            "SELECT percentile_cont($1) WITHIN GROUP (ORDER BY $2) FROM t ORDER BY 1",
        ),
        (
            "SELECT a FROM t GROUP BY 'literal' ORDER BY 1.5",
            "SELECT a FROM t GROUP BY $1 ORDER BY $2",
        ),
    ] {
        assert_normalized(query, expected);
    }
}
