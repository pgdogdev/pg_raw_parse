mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE COLLATION
// Description: define a new collation
// Syntax:
// CREATE COLLATION [ IF NOT EXISTS ] name (
//     [ LOCALE = locale, ]
//     [ LC_COLLATE = lc_collate, ]
//     [ LC_CTYPE = lc_ctype, ]
//     [ PROVIDER = provider, ]
//     [ DETERMINISTIC = boolean, ]
//     [ RULES = rules, ]
//     [ VERSION = version ]
// )
// CREATE COLLATION [ IF NOT EXISTS ] name FROM existing_collation
//
// URL: https://www.postgresql.org/docs/18/sql-createcollation.html

#[test]
fn create_collation_parses() {
    run_cases(&[
        r#"CREATE COLLATION my_collation (provider = libc, locale = 'C')"#,
        r#"CREATE COLLATION IF NOT EXISTS my_collation FROM "C""#,
    ]);
}
