# End-to-end tests

### `e2e_hasura`

Produced by logging all queries executed by Hasura's end-to-end test suite. There are a lot of interesting statements and this test makes sure we can parse (and deparse)
all of them without crashing.

### `postgres_regress`

Run the parser and deparser against Postgres' own regression tests.
