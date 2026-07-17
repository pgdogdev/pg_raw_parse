mod common;

use common::run_parse_walk_cases as run_cases;

#[test]
fn insert_target_variations() {
    run_cases(&[
        "INSERT INTO users DEFAULT VALUES",
        "INSERT INTO public.users DEFAULT VALUES",
        "INSERT INTO users AS u DEFAULT VALUES",
        "INSERT INTO users (id, name) VALUES (1, 'Sage')",
        "INSERT INTO users AS u (id, name) VALUES (1, 'Sage')",
    ]);
}

#[test]
fn insert_with_clause_variations() {
    run_cases(&[
        "WITH incoming AS (SELECT 1 AS id, 'Sage' AS name) INSERT INTO users SELECT * FROM incoming",
        "WITH RECURSIVE incoming(id) AS (SELECT 1 UNION ALL SELECT id + 1 FROM incoming WHERE id < 3) INSERT INTO users (id) SELECT id FROM incoming",
        "WITH incoming AS MATERIALIZED (SELECT 1 AS id) INSERT INTO users (id) SELECT id FROM incoming",
        "WITH incoming AS NOT MATERIALIZED (SELECT 1 AS id) INSERT INTO users (id) SELECT id FROM incoming",
    ]);
}

#[test]
fn insert_value_source_variations() {
    run_cases(&[
        "INSERT INTO users DEFAULT VALUES",
        "INSERT INTO users (id, name) VALUES (1, 'Sage')",
        "INSERT INTO users (id, name) VALUES (1, DEFAULT)",
        "INSERT INTO users (id, name) VALUES (1, 'Sage'), (2, 'River')",
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        "INSERT INTO users (id, name) SELECT id, name FROM incoming",
        "INSERT INTO users SELECT * FROM incoming",
    ]);
}

#[test]
fn insert_overriding_variations() {
    run_cases(&[
        "INSERT INTO users (id) OVERRIDING SYSTEM VALUE VALUES (1)",
        "INSERT INTO users (id) OVERRIDING USER VALUE VALUES (1)",
        "INSERT INTO users (id) OVERRIDING SYSTEM VALUE SELECT id FROM incoming",
        "INSERT INTO users (id) OVERRIDING USER VALUE SELECT id FROM incoming",
    ]);
}

#[test]
fn insert_on_conflict_target_variations() {
    run_cases(&[
        "INSERT INTO users (id) VALUES (1) ON CONFLICT DO NOTHING",
        "INSERT INTO users (id) VALUES (1) ON CONFLICT (id) DO NOTHING",
        "INSERT INTO users (email) VALUES ('a@example.com') ON CONFLICT ((lower(email))) DO NOTHING",
        "INSERT INTO users (email) VALUES ('a@example.com') ON CONFLICT (email COLLATE \"C\") DO NOTHING",
        "INSERT INTO users (email) VALUES ('a@example.com') ON CONFLICT (email text_ops) DO NOTHING",
        "INSERT INTO users (email) VALUES ('a@example.com') ON CONFLICT (email) WHERE active DO NOTHING",
        "INSERT INTO users (email) VALUES ('a@example.com') ON CONFLICT ON CONSTRAINT users_email_key DO NOTHING",
    ]);
}

#[test]
fn insert_on_conflict_action_variations() {
    run_cases(&[
        "INSERT INTO users (id, name) VALUES (1, 'Sage') ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name",
        "INSERT INTO users (id, name) VALUES (1, 'Sage') ON CONFLICT (id) DO UPDATE SET name = DEFAULT",
        "INSERT INTO users (id, name, active) VALUES (1, 'Sage', true) ON CONFLICT (id) DO UPDATE SET (name, active) = ROW (EXCLUDED.name, DEFAULT)",
        "INSERT INTO users (id, name, active) VALUES (1, 'Sage', true) ON CONFLICT (id) DO UPDATE SET (name, active) = (EXCLUDED.name, DEFAULT)",
        "INSERT INTO users (id, name, active) VALUES (1, 'Sage', true) ON CONFLICT (id) DO UPDATE SET (name, active) = (SELECT EXCLUDED.name, EXCLUDED.active)",
        "INSERT INTO users (id, name) VALUES (1, 'Sage') ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name WHERE users.name IS DISTINCT FROM EXCLUDED.name",
    ]);
}

#[test]
fn insert_returning_variations() {
    run_cases(&[
        "INSERT INTO users DEFAULT VALUES RETURNING *",
        "INSERT INTO users (id) VALUES (1) RETURNING id",
        "INSERT INTO users (id) VALUES (1) RETURNING id AS inserted_id, email new_email",
        "INSERT INTO users (id) VALUES (1) RETURNING WITH (OLD AS old_row) id",
        "INSERT INTO users (id) VALUES (1) RETURNING WITH (NEW AS new_row) new_row.id",
        "INSERT INTO users (id) VALUES (1) ON CONFLICT (id) DO UPDATE SET id = EXCLUDED.id RETURNING WITH (OLD AS old_row, NEW AS new_row) old_row.id AS old_id, new_row.id AS new_id",
    ]);
}

// Command:     INSERT
// Description: create new rows in a table
// Syntax:
// [ WITH [ RECURSIVE ] with_query [, ...] ]
// INSERT INTO table_name [ AS alias ] [ ( column_name [, ...] ) ]
//     [ OVERRIDING { SYSTEM | USER } VALUE ]
//     { DEFAULT VALUES | VALUES ( { expression | DEFAULT } [, ...] ) [, ...] | query }
//     [ ON CONFLICT [ conflict_target ] conflict_action ]
//     [ RETURNING [ WITH ( { OLD | NEW } AS output_alias [, ...] ) ]
//                 { * | output_expression [ [ AS ] output_name ] } [, ...] ]

// where conflict_target can be one of:

//     ( { index_column_name | ( index_expression ) } [ COLLATE collation ] [ opclass ] [, ...] ) [ WHERE index_predicate ]
//     ON CONSTRAINT constraint_name

// and conflict_action is one of:

//     DO NOTHING
//     DO UPDATE SET { column_name = { expression | DEFAULT } |
//                     ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
//                     ( column_name [, ...] ) = ( sub-SELECT )
//                   } [, ...]
//               [ WHERE condition ]
//
// URL: https://www.postgresql.org/docs/18/sql-insert.html
