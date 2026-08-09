use flate2::read::GzDecoder;
use pg_raw_parse::{deparse, parse};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[test]
fn hasura_e2e_statements_parse_and_deparse() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/e2e_hasura.sql.gz");
    let file = File::open(&path)
        .unwrap_or_else(|error| panic!("failed to open {}: {error}", path.display()));
    let reader = BufReader::new(GzDecoder::new(file));

    let mut query = String::new();
    let mut records = 0;
    let mut statements = 0;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|error| {
            panic!(
                "failed to decompress {} at line {}: {error}",
                path.display(),
                line_number + 1
            )
        });

        if line.trim() != ";" {
            query.push_str(&line);
            query.push('\n');
            continue;
        }

        if query.trim().is_empty() {
            query.clear();
            continue;
        }

        records += 1;
        parse_and_deparse_record(&query, records, &mut statements);
        query.clear();
    }

    if !query.trim().is_empty() {
        records += 1;
        parse_and_deparse_record(&query, records, &mut statements);
    }

    assert_eq!(records, 113_289, "unexpected Hasura SQL record count");
    assert_eq!(statements, 136_459, "unexpected Hasura SQL statement count");
    eprintln!("Hasura corpus: {records} records, {statements} statements");
}

fn parse_and_deparse_record(query: &str, record: usize, statements: &mut usize) {
    let tree = parse(query)
        .unwrap_or_else(|error| panic!("failed to parse Hasura record {record}: {error}\n{query}"));

    for statement in tree.stmts() {
        *statements += 1;
        let _ = format!("{statement:?}");
        deparse(statement).unwrap_or_else(|error| {
            panic!("failed to deparse Hasura statement {statements}: {error}\n{query}")
        });
    }
}
