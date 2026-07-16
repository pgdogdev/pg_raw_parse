mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_parses() {
    run_cases(&[
        "DROP TABLE IF EXISTS my_table",
        "DROP TABLE IF EXISTS my_table, other_table CASCADE",
    ]);
}

// DROP TABLE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
