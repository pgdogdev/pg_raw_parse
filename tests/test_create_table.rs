mod common;

use common::{run_parse_debug_case as run_test, run_parse_debug_cases as run_cases};
use pg_raw_parse::{Node, parse};

#[test]
fn create_table_prefix_variations() {
    run_cases(&[
        "CREATE TABLE users ()",
        "CREATE TABLE public.users ()",
        "CREATE TEMP TABLE users ()",
        "CREATE TEMPORARY TABLE users ()",
        "CREATE LOCAL TEMP TABLE users ()",
        "CREATE LOCAL TEMPORARY TABLE users ()",
        "CREATE GLOBAL TEMP TABLE users ()",
        "CREATE GLOBAL TEMPORARY TABLE users ()",
        "CREATE UNLOGGED TABLE users ()",
        "CREATE TABLE IF NOT EXISTS users ()",
        "CREATE TEMP TABLE IF NOT EXISTS users ()",
        "CREATE UNLOGGED TABLE IF NOT EXISTS users ()",
    ]);
}

#[test]
fn create_table_column_variations() {
    run_cases(&[
        "CREATE TABLE users (id integer)",
        "CREATE TABLE users (id integer STORAGE PLAIN)",
        "CREATE TABLE users (id integer STORAGE EXTERNAL)",
        "CREATE TABLE users (id integer STORAGE EXTENDED)",
        "CREATE TABLE users (id integer STORAGE MAIN)",
        "CREATE TABLE users (id integer STORAGE DEFAULT)",
        "CREATE TABLE users (body text COMPRESSION pglz)",
        "CREATE TABLE users (name text COLLATE \"C\")",
        "CREATE TABLE users (id integer DEFAULT 1)",
        "CREATE TABLE users (id integer CONSTRAINT users_id_default DEFAULT 1)",
        "CREATE TABLE users (id integer CONSTRAINT users_id_nn NOT NULL)",
        "CREATE TABLE users (name text NULL)",
        "CREATE TABLE users (email text UNIQUE)",
        "CREATE TABLE users (email text UNIQUE NULLS DISTINCT)",
        "CREATE TABLE users (email text UNIQUE NULLS NOT DISTINCT)",
        "CREATE TABLE users (email text UNIQUE WITH (fillfactor = 70))",
        "CREATE TABLE users (email text UNIQUE USING INDEX TABLESPACE fastspace)",
        "CREATE TABLE users (id integer PRIMARY KEY)",
        "CREATE TABLE users (id integer PRIMARY KEY WITH (fillfactor = 70))",
        "CREATE TABLE users (id integer PRIMARY KEY USING INDEX TABLESPACE fastspace)",
        "CREATE TABLE users (age integer CHECK (age > 0))",
        "CREATE TABLE users (age integer CHECK (age > 0) NO INHERIT)",
        "CREATE TABLE users (normalized text GENERATED ALWAYS AS (lower(name)) STORED)",
        "CREATE TABLE users (account_id integer REFERENCES accounts)",
        "CREATE TABLE users (account_id integer REFERENCES accounts (id))",
        "CREATE TABLE users (account_id integer REFERENCES accounts (id) MATCH FULL)",
        "CREATE TABLE users (account_id integer REFERENCES accounts (id) MATCH SIMPLE)",
        "CREATE TABLE users (account_id integer REFERENCES accounts (id) ON DELETE CASCADE ON UPDATE SET NULL)",
        "CREATE TABLE users (account_id integer REFERENCES accounts (id) DEFERRABLE INITIALLY DEFERRED)",
        "CREATE TABLE users (account_id integer REFERENCES accounts (id) NOT DEFERRABLE INITIALLY IMMEDIATE)",
    ]);
}

#[test]
fn create_table_constraint_variations() {
    run_cases(&[
        "CREATE TABLE users (id integer, email text, UNIQUE (email))",
        "CREATE TABLE users (id integer, email text, UNIQUE NULLS DISTINCT (email))",
        "CREATE TABLE users (id integer, email text, UNIQUE NULLS NOT DISTINCT (email))",
        "CREATE TABLE users (id integer, email text, UNIQUE (email) INCLUDE (id))",
        "CREATE TABLE users (id integer, email text, UNIQUE (email) WITH (fillfactor = 70))",
        "CREATE TABLE users (id integer, email text, CONSTRAINT users_email_key UNIQUE (email) USING INDEX TABLESPACE fastspace)",
        "CREATE TABLE users (id integer, PRIMARY KEY (id))",
        "CREATE TABLE users (id integer, email text, PRIMARY KEY (id) INCLUDE (email))",
        "CREATE TABLE users (id integer, PRIMARY KEY (id) WITH (fillfactor = 70))",
        "CREATE TABLE users (id integer, CONSTRAINT users_pkey PRIMARY KEY (id) USING INDEX TABLESPACE fastspace)",
        "CREATE TABLE users (age integer, CHECK (age > 0))",
        "CREATE TABLE users (age integer, CHECK (age > 0) NO INHERIT)",
        "CREATE TABLE users (room int4range, EXCLUDE USING gist (room WITH &&))",
        "CREATE TABLE users (room int4range, during tstzrange, EXCLUDE USING gist (room WITH &&, during WITH &&) WHERE (room IS NOT NULL))",
        "CREATE TABLE users (room int4range, EXCLUDE (room WITH &&) INCLUDE (room) WITH (fillfactor = 70) USING INDEX TABLESPACE fastspace)",
        "CREATE TABLE users (account_id integer, FOREIGN KEY (account_id) REFERENCES accounts)",
        "CREATE TABLE users (account_id integer, tenant_id integer, FOREIGN KEY (account_id, tenant_id) REFERENCES accounts (id, tenant_id))",
        "CREATE TABLE users (account_id integer, FOREIGN KEY (account_id) REFERENCES accounts (id) MATCH FULL ON DELETE CASCADE ON UPDATE RESTRICT)",
        "CREATE TABLE users (account_id integer, tenant_id integer, FOREIGN KEY (account_id, tenant_id) REFERENCES accounts (id, tenant_id) ON DELETE SET NULL (account_id) ON UPDATE SET DEFAULT)",
        "CREATE TABLE users (account_id integer, FOREIGN KEY (account_id) REFERENCES accounts (id) MATCH SIMPLE ON DELETE NO ACTION ON UPDATE NO ACTION DEFERRABLE INITIALLY DEFERRED)",
        "CREATE TABLE users (account_id integer, CONSTRAINT users_account_fk FOREIGN KEY (account_id) REFERENCES accounts (id) NOT DEFERRABLE INITIALLY IMMEDIATE)",
    ]);
}

#[test]
fn create_table_like_variations() {
    run_cases(&[
        "CREATE TABLE users (LIKE old_users)",
        "CREATE TABLE users (LIKE old_users INCLUDING COMMENTS)",
        "CREATE TABLE users (LIKE old_users INCLUDING COMPRESSION)",
        "CREATE TABLE users (LIKE old_users INCLUDING CONSTRAINTS)",
        "CREATE TABLE users (LIKE old_users INCLUDING DEFAULTS)",
        "CREATE TABLE users (LIKE old_users INCLUDING GENERATED)",
        "CREATE TABLE users (LIKE old_users INCLUDING IDENTITY)",
        "CREATE TABLE users (LIKE old_users INCLUDING INDEXES)",
        "CREATE TABLE users (LIKE old_users INCLUDING STATISTICS)",
        "CREATE TABLE users (LIKE old_users INCLUDING STORAGE)",
        "CREATE TABLE users (LIKE old_users INCLUDING ALL)",
        "CREATE TABLE users (LIKE old_users EXCLUDING COMMENTS)",
        "CREATE TABLE users (LIKE old_users EXCLUDING COMPRESSION)",
        "CREATE TABLE users (LIKE old_users EXCLUDING CONSTRAINTS)",
        "CREATE TABLE users (LIKE old_users EXCLUDING DEFAULTS)",
        "CREATE TABLE users (LIKE old_users EXCLUDING GENERATED)",
        "CREATE TABLE users (LIKE old_users EXCLUDING IDENTITY)",
        "CREATE TABLE users (LIKE old_users EXCLUDING INDEXES)",
        "CREATE TABLE users (LIKE old_users EXCLUDING STATISTICS)",
        "CREATE TABLE users (LIKE old_users EXCLUDING STORAGE)",
        "CREATE TABLE users (LIKE old_users EXCLUDING ALL)",
        "CREATE TABLE users (LIKE old_users INCLUDING DEFAULTS EXCLUDING INDEXES)",
        "CREATE TABLE users (id integer, LIKE old_users INCLUDING DEFAULTS, email text)",
    ]);
}

#[test]
fn create_table_options_variations() {
    run_cases(&[
        "CREATE TABLE users (id integer) INHERITS (base_users)",
        "CREATE TABLE users (id integer) INHERITS (base_users, audited_rows)",
        "CREATE TABLE users (id integer) PARTITION BY RANGE (created_at)",
        "CREATE TABLE users (id integer) PARTITION BY RANGE ((lower(name)) COLLATE \"C\" text_ops)",
        "CREATE TABLE users (id integer) PARTITION BY LIST (tenant_id)",
        "CREATE TABLE users (id integer) PARTITION BY HASH (tenant_id)",
        "CREATE TABLE users (id integer) USING heap",
        "CREATE TABLE users (id integer) WITH (fillfactor = 70)",
        "CREATE TABLE users (id integer) WITH (fillfactor)",
        "CREATE TABLE users (id integer) WITHOUT OIDS",
        "CREATE TEMP TABLE users (id integer) ON COMMIT PRESERVE ROWS",
        "CREATE TEMP TABLE users (id integer) ON COMMIT DELETE ROWS",
        "CREATE TEMP TABLE users (id integer) ON COMMIT DROP",
        "CREATE TABLE users (id integer) TABLESPACE fastspace",
        "CREATE TEMP TABLE users (id integer) INHERITS (base_users) WITHOUT OIDS ON COMMIT DELETE ROWS TABLESPACE fastspace",
    ]);
}

#[test]
fn create_typed_table_variations() {
    run_cases(&[
        "CREATE TABLE users OF user_type",
        "CREATE TABLE users OF user_type (id WITH OPTIONS NOT NULL)",
        "CREATE TABLE users OF user_type (id WITH OPTIONS DEFAULT 1, CHECK (id > 0))",
        "CREATE TABLE users OF user_type PARTITION BY HASH (id)",
        "CREATE TABLE users OF user_type USING heap WITH (fillfactor = 70) TABLESPACE fastspace",
    ]);
}

#[test]
fn create_partition_table_variations() {
    run_cases(&[
        "CREATE TABLE users_default PARTITION OF users DEFAULT",
        "CREATE TABLE users_us PARTITION OF users FOR VALUES IN ('us', 'ca')",
        "CREATE TABLE users_hash_0 PARTITION OF users FOR VALUES WITH (MODULUS 4, REMAINDER 0)",
        "CREATE TABLE users_hash_0 PARTITION OF users (id WITH OPTIONS NOT NULL) FOR VALUES WITH (MODULUS 4, REMAINDER 0)",
        "CREATE TABLE users_us PARTITION OF users FOR VALUES IN ('us', 'ca') PARTITION BY HASH (tenant_id)",
        "CREATE TABLE users_us PARTITION OF users FOR VALUES IN ('us', 'ca') USING heap WITH (fillfactor = 70) TABLESPACE fastspace",
    ]);
}

#[test]
fn create_table_identity_variations() {
    run_cases(&[
        "CREATE TABLE users (id integer GENERATED ALWAYS AS IDENTITY)",
        "CREATE TABLE users (id integer GENERATED BY DEFAULT AS IDENTITY)",
        "CREATE TABLE users (id integer GENERATED ALWAYS AS IDENTITY (START WITH 10 INCREMENT BY 2))",
    ]);
}

#[test]
fn create_table_exclude_element_variations() {
    run_cases(&[
        "CREATE TABLE bookings (room int4range, EXCLUDE USING gist (room WITH &&))",
        "CREATE TABLE bookings (room text, EXCLUDE USING gist (room COLLATE \"C\" text_ops WITH =))",
        "CREATE TABLE bookings (room text, EXCLUDE USING gist ((lower(room)) WITH =))",
        "CREATE TABLE bookings (room int, EXCLUDE USING btree (room ASC NULLS LAST WITH =))",
    ]);
}

#[test]
#[should_panic(expected = "MATCH PARTIAL not yet implemented")]
fn failing_create_table_column_reference_match_partial() {
    run_test("CREATE TABLE users (account_id integer REFERENCES accounts (id) MATCH PARTIAL)");
}

#[test]
#[should_panic(expected = "MATCH PARTIAL not yet implemented")]
fn failing_create_table_table_reference_match_partial() {
    run_test(
        "CREATE TABLE users (account_id integer, FOREIGN KEY (account_id) REFERENCES accounts (id) MATCH PARTIAL ON DELETE SET NULL ON UPDATE SET DEFAULT)",
    );
}

#[test]
fn create_table_column_not_null_no_inherit() {
    run_test("CREATE TABLE users (id integer NOT NULL NO INHERIT)");
}

#[test]
fn create_table_partition_of_range_from_to() {
    run_test(
        "CREATE TABLE users_2024 PARTITION OF users FOR VALUES FROM ('2024-01-01') TO ('2025-01-01')",
    );
}

#[test]
fn create_table_partition_of_range_minvalue_maxvalue() {
    run_test(
        "CREATE TABLE users_2024 PARTITION OF users (id WITH OPTIONS NOT NULL) FOR VALUES FROM (MINVALUE) TO (MAXVALUE)",
    );
}

#[test]
fn create_table_partition_of_range_with_partition_by() {
    let ast = parse(
        "CREATE TABLE users_2024 PARTITION OF users FOR VALUES FROM ('2024-01-01') TO ('2025-01-01') PARTITION BY HASH (tenant_id)",
    )
    .unwrap();

    // Range partition bounds are exposed through CreateStmt::partbound().
    // lowerdatums()/upperdatums() are generic NodeLists because raw parse
    // trees can contain A_Const nodes for concrete values.
    let Node::CreateStmt(stmt) = ast.stmts().next().unwrap() else {
        panic!("expected CREATE TABLE statement");
    };
    let bounds = stmt.partbound().expect("expected partition bounds");
    let lower = bounds.lowerdatums().first().expect("expected lower bound");
    let upper = bounds.upperdatums().first().expect("expected upper bound");

    assert!(matches!(lower, Node::A_Const(_)));
    assert!(matches!(upper, Node::A_Const(_)));
    assert!(!format!("{bounds:?}").is_empty());
}

#[test]
fn create_table_partition_of_range_with_options() {
    run_test(
        "CREATE TABLE users_2024 PARTITION OF users FOR VALUES FROM ('2024-01-01') TO ('2025-01-01') USING heap WITH (fillfactor = 70) TABLESPACE fastspace",
    );
}

// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name ( [
//   { column_name data_type [ STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT } ] [ COMPRESSION compression_method ] [ COLLATE collation ] [ column_constraint [ ... ] ]
//     | table_constraint
//     | LIKE source_table [ like_option ... ] }
//     [, ... ]
// ] )
// [ INHERITS ( parent_table [, ... ] ) ]
// [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
// [ USING method ]
// [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
// [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
// [ TABLESPACE tablespace_name ]

// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name
//     OF type_name [ (
//   { column_name [ WITH OPTIONS ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ) ]
// [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
// [ USING method ]
// [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
// [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
// [ TABLESPACE tablespace_name ]

// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name
//     PARTITION OF parent_table [ (
//   { column_name [ WITH OPTIONS ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ) ] { FOR VALUES partition_bound_spec | DEFAULT }
// [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
// [ USING method ]
// [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
// [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
// [ TABLESPACE tablespace_name ]

// where column_constraint is:

// [ CONSTRAINT constraint_name ]
// { NOT NULL [ NO INHERIT ]  |
//   NULL |
//   CHECK ( expression ) [ NO INHERIT ] |
//   DEFAULT default_expr |
//   GENERATED ALWAYS AS ( generation_expr ) [ STORED | VIRTUAL ] |
//   GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] index_parameters |
//   PRIMARY KEY index_parameters |
//   REFERENCES reftable [ ( refcolumn ) ] [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ]
//     [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
// [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ] [ ENFORCED | NOT ENFORCED ]

// and table_constraint is:

// [ CONSTRAINT constraint_name ]
// { CHECK ( expression ) [ NO INHERIT ] |
//   NOT NULL column_name [ NO INHERIT ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] ( column_name [, ... ] [, column_name WITHOUT OVERLAPS ] ) index_parameters |
//   PRIMARY KEY ( column_name [, ... ] [, column_name WITHOUT OVERLAPS ] ) index_parameters |
//   EXCLUDE [ USING index_method ] ( exclude_element WITH operator [, ... ] ) index_parameters [ WHERE ( predicate ) ] |
//   FOREIGN KEY ( column_name [, ... ] [, PERIOD column_name ] ) REFERENCES reftable [ ( refcolumn [, ... ] [, PERIOD refcolumn ] ) ]
//     [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ] [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
// [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ] [ ENFORCED | NOT ENFORCED ]

// and like_option is:

// { INCLUDING | EXCLUDING } { COMMENTS | COMPRESSION | CONSTRAINTS | DEFAULTS | GENERATED | IDENTITY | INDEXES | STATISTICS | STORAGE | ALL }

// and partition_bound_spec is:

// IN ( partition_bound_expr [, ...] ) |
// FROM ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] )
//   TO ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] ) |
// WITH ( MODULUS numeric_literal, REMAINDER numeric_literal )

// index_parameters in UNIQUE, PRIMARY KEY, and EXCLUDE constraints are:

// [ INCLUDE ( column_name [, ... ] ) ]
// [ WITH ( storage_parameter [= value] [, ... ] ) ]
// [ USING INDEX TABLESPACE tablespace_name ]

// exclude_element in an EXCLUDE constraint is:

// { column_name | ( expression ) } [ COLLATE collation ] [ opclass [ ( opclass_parameter = value [, ... ] ) ] ] [ ASC | DESC ] [ NULLS { FIRST | LAST } ]

// referential_action in a FOREIGN KEY/REFERENCES constraint is:

// { NO ACTION | RESTRICT | CASCADE | SET NULL [ ( column_name [, ... ] ) ] | SET DEFAULT [ ( column_name [, ... ] ) ] }
