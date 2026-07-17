mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER SUBSCRIPTION
// Description: change the definition of a subscription
// Syntax:
// ALTER SUBSCRIPTION name CONNECTION 'conninfo'
// ALTER SUBSCRIPTION name SET PUBLICATION publication_name [, ...] [ WITH ( publication_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name ADD PUBLICATION publication_name [, ...] [ WITH ( publication_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name DROP PUBLICATION publication_name [, ...] [ WITH ( publication_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name REFRESH PUBLICATION [ WITH ( refresh_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name ENABLE
// ALTER SUBSCRIPTION name DISABLE
// ALTER SUBSCRIPTION name SET ( subscription_parameter [= value] [, ... ] )
// ALTER SUBSCRIPTION name SKIP ( skip_option = value )
// ALTER SUBSCRIPTION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER SUBSCRIPTION name RENAME TO new_name
//
// URL: https://www.postgresql.org/docs/18/sql-altersubscription.html

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
