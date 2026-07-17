#![allow(dead_code)]

use pg_raw_parse::{parse, walk::walk};

pub fn run_parse_debug_case(query: &str) {
    let ast = parse(query).unwrap_or_else(|err| panic!("failed to parse `{query}`: {err:?}"));
    assert!(!format!("{:?}", ast).is_empty())
}

pub fn run_parse_debug_cases(cases: &[&str]) {
    for query in cases {
        run_parse_debug_case(query);
    }
}

pub fn run_parse_walk_case(query: &str) {
    let ast = parse(query).unwrap_or_else(|err| panic!("failed to parse `{query}`: {err:?}"));
    walk(ast.stmts().next().unwrap(), |node| {
        assert!(!format!("{:?}", node).is_empty());
    });
    assert!(!format!("{:?}", ast).is_empty())
}

pub fn run_parse_walk_cases(cases: &[&str]) {
    for query in cases {
        run_parse_walk_case(query);
    }
}
