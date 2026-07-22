# PG Raw Parse
## Safe bindings to libpg_query

PG Raw Parse provides a low level wrapper around the PostgreSQL backend parser.
These bindings, as well as some additional functionality are provided by
[libpg\_query](https://github.com/pganalyze/libpg_query).

In addition to parsing, we provide mechanisms to [traverse an AST], [construct
new ASTs], and [transform ASTs]. See the API docs for more details.

[traverse an AST]: https://docs.rs/pg_raw_parse/latest/pg_raw_parse/walk/index.html
[construct new ASTs]: https://docs.rs/pg_raw_parse/latest/pg_raw_parse/make/index.html
[transform ASTs]: https://docs.rs/pg_raw_parse/latest/pg_raw_parse/transform/index.html

This library's API surface is primarily driven by the needs of
[PgDog](https://github.com/pgdogdev/pgdog). It is not intended to be a complete,
one-size-fits-all solution to PostgreSQL ASTs.  Contributions are welcome, but
pull requests adding large and complex features are unlikely to be accepted
unless they align with PgDog's needs. For a more general purpose library,
consider [pg\_query.rs](https://github.com/pganalyze/pg_query.rs).
