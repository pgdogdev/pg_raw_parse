#[cfg(all(feature = "pg_query", feature = "pg_raw_parse"))]
compile_error!("features `pg_query` and `pg_raw_parse` are mutually exclusive; enable only one");

#[cfg(any(feature = "pg_query", feature = "pg_raw_parse"))]
fn main() {
    use criterion::{BenchmarkId, Criterion, Throughput};
    use std::fmt::Write;
    use std::hint::black_box;

    #[cfg(feature = "pg_query")]
    use pg_query as parser;
    #[cfg(all(feature = "pg_raw_parse", not(feature = "pg_query")))]
    use pg_raw_parse as parser;

    let parser_name = if cfg!(feature = "pg_query") {
        "pg_query"
    } else {
        "pg_raw_parse"
    };
    let mut criterion = Criterion::default().configure_from_args();
    let mut group = criterion.benchmark_group(format!("{parser_name}::parse"));
    // One element is one complete parse, so elem/s is parses per second.
    group.throughput(Throughput::Elements(1));

    for nodes in [10, 100, 1_000, 2_000, 5_000, 10_000] {
        // Each IN-list value adds an A_Const node; the rest of the AST is fixed.
        // Generate the SQL once, outside the timed loop.
        let mut sql = String::from("SELECT * FROM users WHERE id IN (");
        for id in 0..nodes {
            if id > 0 {
                sql.push(',');
            }
            write!(sql, "{id}").unwrap();
        }
        sql.push(')');

        group.bench_with_input(BenchmarkId::from_parameter(nodes), &sql, |b, sql| {
            b.iter(|| parser::parse(black_box(sql.as_str())).unwrap());
        });
    }

    group.finish();
    criterion.final_summary();
}

#[cfg(not(any(feature = "pg_query", feature = "pg_raw_parse")))]
fn main() {
    eprintln!("Enable one parser feature: --features pg_query or --features pg_raw_parse");
    std::process::exit(1);
}
