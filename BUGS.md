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
