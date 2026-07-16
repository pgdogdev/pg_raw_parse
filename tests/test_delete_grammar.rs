mod common;

use common::run_parse_walk_cases as run_cases;

#[test]
fn delete_target_variations() {
    run_cases(&[
        "DELETE FROM users",
        "DELETE FROM ONLY users",
        "DELETE FROM users *",
        "DELETE FROM users AS u",
        "DELETE FROM users u",
        "DELETE FROM public.users AS u",
    ]);
}

#[test]
fn delete_with_clause_variations() {
    run_cases(&[
        "WITH stale AS (SELECT id FROM users WHERE active = false) DELETE FROM users USING stale WHERE users.id = stale.id",
        "WITH RECURSIVE stale(id) AS (SELECT 1 UNION ALL SELECT id + 1 FROM stale WHERE id < 3) DELETE FROM users USING stale WHERE users.id = stale.id",
        "WITH stale AS MATERIALIZED (SELECT id FROM users) DELETE FROM users USING stale WHERE users.id = stale.id",
        "WITH stale AS NOT MATERIALIZED (SELECT id FROM users) DELETE FROM users USING stale WHERE users.id = stale.id",
    ]);
}

#[test]
fn delete_using_variations() {
    run_cases(&[
        "DELETE FROM users USING orders WHERE users.id = orders.user_id",
        "DELETE FROM users USING orders o, sessions s WHERE users.id = o.user_id AND users.id = s.user_id",
        "DELETE FROM users USING ONLY archived_users au WHERE users.id = au.id",
        "DELETE FROM users USING LATERAL (SELECT 1 AS id) s WHERE users.id = s.id",
        "DELETE FROM users USING generate_series(1, 3) AS g(id) WHERE users.id = g.id",
        "DELETE FROM users USING orders JOIN invoices ON orders.id = invoices.order_id WHERE users.id = orders.user_id",
    ]);
}

#[test]
fn delete_where_variations() {
    run_cases(&[
        "DELETE FROM users WHERE id = 1",
        "DELETE FROM users WHERE id = $1 AND active = $2",
        "DELETE FROM users WHERE EXISTS (SELECT 1 FROM orders WHERE orders.user_id = users.id)",
        "DELETE FROM users WHERE CURRENT OF user_cursor",
    ]);
}

#[test]
fn delete_returning_variations() {
    run_cases(&[
        "DELETE FROM users RETURNING *",
        "DELETE FROM users RETURNING id",
        "DELETE FROM users RETURNING id AS deleted_id, email old_email",
    ]);
}

// [ WITH [ RECURSIVE ] with_query [, ...] ]
// DELETE FROM [ ONLY ] table_name [ * ] [ [ AS ] alias ]
//     [ USING from_item [, ...] ]
//     [ WHERE condition | WHERE CURRENT OF cursor_name ]
//     [ RETURNING [ WITH ( { OLD | NEW } AS output_alias [, ...] ) ]
//                 { * | output_expression [ [ AS ] output_name ] } [, ...] ]
