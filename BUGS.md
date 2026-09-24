# Known bugs

## Function parameters named after keywords

```sql
CREATE FUNCTION f("offset" integer) RETURNS integer
LANGUAGE sql AS $$ SELECT "offset" $$;
```

libpg_query deparses the argument as `offset int`, which is invalid because
`OFFSET` is reserved. `deparseFunctionParameter` in
`libpg_query/src/postgres_deparse.c` should call `quote_identifier` for parameter
names.

## Missing parentheses around the `AT TIME ZONE` zone operand

```sql
SELECT ts AT TIME ZONE ((summary -> 'stop') ->> 'tz') FROM s;
```

libpg_query deparses this as `ts AT TIME ZONE (summary -> 'stop') ->> 'tz'`, then
drops the remaining parentheses on the next round trip. `AT` binds tighter than
`Op`, so the result parses as `(ts AT TIME ZONE summary) -> 'stop' ->> 'tz'` and
fails with `function pg_catalog.timezone(jsonb, timestamp with time zone) does
not exist`.

The `pg_catalog.timezone` branch of `deparseFuncCall` in
`libpg_query/src/postgres_deparse.c` parenthesises the timestamp operand when it
is an `A_Expr`, but writes the zone operand without that check. The zone operand
needs the same `IsA(zone, A_Expr)` parentheses.

`test_deparse` in `src/deparse.rs` covers this. The test fails until libpg_query
ships the fix.
