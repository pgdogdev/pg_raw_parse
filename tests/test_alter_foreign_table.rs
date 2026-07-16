mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn alter_foreign_table_parses() {
    run_cases(&["ALTER FOREIGN TABLE my_foreign_table RENAME TO my_foreign_table2"]);
}
