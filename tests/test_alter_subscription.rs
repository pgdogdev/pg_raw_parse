mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_subscription_parses() {
    run_cases(&[
        r#"ALTER SUBSCRIPTION sub CONNECTION 'host=localhost dbname=postgres'"#,
        r#"ALTER SUBSCRIPTION sub SET PUBLICATION pub1, pub2 WITH (copy_data = false)"#,
        r#"ALTER SUBSCRIPTION sub ADD PUBLICATION pub3 WITH (copy_data = true)"#,
        r#"ALTER SUBSCRIPTION sub DROP PUBLICATION pub3 WITH (refresh = false)"#,
        r#"ALTER SUBSCRIPTION sub REFRESH PUBLICATION WITH (copy_data = false)"#,
        r#"ALTER SUBSCRIPTION sub ENABLE"#,
        r#"ALTER SUBSCRIPTION sub DISABLE"#,
        r#"ALTER SUBSCRIPTION sub SET (slot_name = 'slot1', synchronous_commit = off)"#,
        r#"ALTER SUBSCRIPTION sub SKIP (lsn = '0/16B6C50')"#,
        r#"ALTER SUBSCRIPTION sub OWNER TO CURRENT_USER"#,
        r#"ALTER SUBSCRIPTION sub RENAME TO sub_new"#,
    ]);
}
