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

    // Reuse Criterion's positional filter as the required benchmark selector.
    let benchmark = match std::env::args().nth(1).as_deref() {
        Some("parse") => "parse",
        Some("deparse") => "deparse",
        Some("normalize") => "normalize",
        Some("normalize_str") => "normalize_str",
        _ => {
            eprintln!(
                "Usage: benchmarks <parse|deparse|normalize|normalize_str> [Criterion options]"
            );
            std::process::exit(2);
        }
    };

    let parser_name = if cfg!(feature = "pg_query") {
        "pg_query"
    } else {
        "pg_raw_parse"
    };
    let mut criterion = Criterion::default().configure_from_args();
    let mut group = criterion.benchmark_group(format!("{parser_name}::{benchmark}"));
    // One element is one complete operation, so elem/s is operations per second.
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

        if benchmark == "parse" {
            group.bench_with_input(BenchmarkId::from_parameter(nodes), &sql, |b, sql| {
                b.iter(|| parser::parse(black_box(sql.as_str())).unwrap());
            });
        } else if benchmark == "normalize_str" {
            // Include parsing, normalization, and SQL output in each iteration.
            group.bench_with_input(BenchmarkId::from_parameter(nodes), &sql, |b, sql| {
                #[cfg(feature = "pg_query")]
                b.iter(|| parser::normalize(black_box(sql.as_str())).unwrap());

                #[cfg(all(feature = "pg_raw_parse", not(feature = "pg_query")))]
                b.iter(|| parser::normalize::normalize_str(black_box(sql.as_str())).unwrap());
            });
        } else if benchmark == "normalize" {
            #[cfg(feature = "pg_query")]
            group.bench_with_input(BenchmarkId::from_parameter(nodes), &sql, |b, sql| {
                b.iter(|| parser::normalize(black_box(sql.as_str())).unwrap());
            });

            #[cfg(all(feature = "pg_raw_parse", not(feature = "pg_query")))]
            {
                // Normalize the AST directly, excluding parsing and deparsing.
                let parsed = parser::parse(&sql).unwrap();
                let stmt = parsed.first().unwrap();
                group.bench_function(BenchmarkId::from_parameter(nodes), |b| {
                    b.iter(|| parser::normalize::normalize(black_box(stmt)));
                });
            }
        } else {
            // Parse once so deparse timing excludes constructing the AST.
            let parsed = parser::parse(&sql).unwrap();
            group.bench_with_input(BenchmarkId::from_parameter(nodes), &parsed, |b, parsed| {
                #[cfg(feature = "pg_query")]
                b.iter(|| parser::deparse(black_box(&parsed.protobuf)).unwrap());

                #[cfg(all(feature = "pg_raw_parse", not(feature = "pg_query")))]
                {
                    let stmt = parsed.first().unwrap();
                    b.iter(|| parser::deparse(black_box(stmt)).unwrap());
                }
            });
        }
    }

    group.finish();
    criterion.final_summary();
}

#[cfg(not(any(feature = "pg_query", feature = "pg_raw_parse")))]
fn main() {
    eprintln!("Enable one parser feature: --features pg_query or --features pg_raw_parse");
    std::process::exit(1);
}
