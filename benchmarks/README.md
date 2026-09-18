# Parse benchmarks

Run each parser separately in the optimized benchmark profile:

```sh
cargo bench --manifest-path benchmarks/Cargo.toml --bin benchmarks --features pg_raw_parse
cargo bench --manifest-path benchmarks/Cargo.toml --bin benchmarks --features pg_query
```

Neither parser is enabled by default; enabling both is a compile-time error.

Criterion prints time per iteration and throughput. One iteration parses one
complete query, so `elem/s` means parses per second (`Kelem/s` means thousands).
The timed operation includes dropping the parse result. Query generation is
excluded from timing, and both parsers receive identical SQL.

The generated query is `SELECT * FROM users WHERE id IN (0,1,...)`, with
10, 100, 1,000, 2,000, 5,000, and 10,000 integer literal nodes in the `IN` list.
These sizes count the added literal nodes, not the query's fixed AST overhead.
Each benchmark calls the selected crate's public `parse` method.

For a quick smoke test, append `-- --test` to either command. To run through
`cargo run` instead, use `--release` and append `-- --bench` to enable measurement.
