mod common;

use common::run_parse_walk_cases as run_cases;

#[test]
fn update_target_variations() {
    run_cases(&[
        "UPDATE users SET name = 'Sage'",
        "UPDATE ONLY users SET name = 'Sage'",
        "UPDATE users * SET name = 'Sage'",
        "UPDATE users AS u SET name = 'Sage'",
        "UPDATE users u SET name = 'Sage'",
        "UPDATE public.users AS u SET name = 'Sage'",
    ]);
}

#[test]
fn update_with_clause_variations() {
    run_cases(&[
        "WITH incoming AS (SELECT 1 AS id, 'Sage' AS name) UPDATE users SET name = incoming.name FROM incoming WHERE users.id = incoming.id",
        "WITH RECURSIVE incoming(id) AS (SELECT 1 UNION ALL SELECT id + 1 FROM incoming WHERE id < 3) UPDATE users SET active = true FROM incoming WHERE users.id = incoming.id",
        "WITH incoming AS MATERIALIZED (SELECT 1 AS id) UPDATE users SET active = true FROM incoming WHERE users.id = incoming.id",
        "WITH incoming AS NOT MATERIALIZED (SELECT 1 AS id) UPDATE users SET active = true FROM incoming WHERE users.id = incoming.id",
    ]);
}

#[test]
fn update_set_variations() {
    run_cases(&[
        "UPDATE users SET name = 'Sage'",
        "UPDATE users SET name = DEFAULT",
        "UPDATE users SET name = $1, active = $2",
        "UPDATE users SET (name, active) = ROW ('Sage', true)",
        "UPDATE users SET (name, active) = ('Sage', DEFAULT)",
        "UPDATE users SET (name, active) = (SELECT name, active FROM incoming WHERE incoming.id = users.id)",
        "UPDATE users SET name = upper(name), updated_at = now()",
    ]);
}

#[test]
fn update_from_variations() {
    run_cases(&[
        "UPDATE users SET active = false FROM orders WHERE users.id = orders.user_id",
        "UPDATE users SET active = false FROM orders o, sessions s WHERE users.id = o.user_id AND users.id = s.user_id",
        "UPDATE users SET active = false FROM LATERAL (SELECT 1 AS id) s WHERE users.id = s.id",
        "UPDATE users SET active = false FROM generate_series(1, 3) AS g(id) WHERE users.id = g.id",
        "UPDATE users SET active = false FROM orders JOIN invoices ON orders.id = invoices.order_id WHERE users.id = orders.user_id",
    ]);
}

#[test]
fn update_where_variations() {
    run_cases(&[
        "UPDATE users SET active = false WHERE id = 1",
        "UPDATE users SET active = $1 WHERE id = $2",
        "UPDATE users SET active = false WHERE EXISTS (SELECT 1 FROM orders WHERE orders.user_id = users.id)",
        "UPDATE users SET active = false WHERE CURRENT OF user_cursor",
    ]);
}

#[test]
fn update_returning_variations() {
    run_cases(&[
        "UPDATE users SET active = false RETURNING *",
        "UPDATE users SET active = false RETURNING id",
        "UPDATE users SET active = false RETURNING id AS updated_id, email new_email",
        "UPDATE users SET active = false RETURNING WITH (OLD AS old_row) old_row.active",
        "UPDATE users SET active = true RETURNING WITH (NEW AS new_row) new_row.active",
        "UPDATE users SET active = true RETURNING WITH (OLD AS old_row, NEW AS new_row) old_row.active AS was_active, new_row.active AS is_active",
    ]);
}
