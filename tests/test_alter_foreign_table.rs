mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_foreign_table_parses() {
    run_cases(&[
        r#"ALTER FOREIGN TABLE IF EXISTS ONLY ft ADD COLUMN IF NOT EXISTS payload text OPTIONS (column_name 'payload')"#,
        r#"ALTER FOREIGN TABLE ft DROP COLUMN IF EXISTS old_col CASCADE"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload TYPE varchar(200)"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload SET DEFAULT 'x'"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload DROP DEFAULT"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload SET NOT NULL"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload DROP NOT NULL"#,
        r#"ALTER FOREIGN TABLE ft ALTER COLUMN payload SET STATISTICS 100"#,
        r#"ALTER FOREIGN TABLE ft ADD CHECK (payload <> '') NOT VALID"#,
        r#"ALTER FOREIGN TABLE ft VALIDATE CONSTRAINT payload_check"#,
        r#"ALTER FOREIGN TABLE ft DROP CONSTRAINT IF EXISTS payload_check RESTRICT"#,
        r#"ALTER FOREIGN TABLE ft DISABLE TRIGGER ALL"#,
        r#"ALTER FOREIGN TABLE ft ENABLE ALWAYS TRIGGER trg"#,
        r#"ALTER FOREIGN TABLE ft SET WITHOUT OIDS"#,
        r#"ALTER FOREIGN TABLE ft OWNER TO CURRENT_USER"#,
        r#"ALTER FOREIGN TABLE ft OPTIONS (ADD schema_name 'public', SET table_name 'remote_table')"#,
        r#"ALTER FOREIGN TABLE ft RENAME COLUMN payload TO body"#,
        r#"ALTER FOREIGN TABLE ft RENAME TO ft_new"#,
        r#"ALTER FOREIGN TABLE ft SET SCHEMA archive"#,
    ]);
}
