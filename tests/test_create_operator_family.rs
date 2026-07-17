mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE OPERATOR FAMILY
// Description: define a new operator family
// Syntax:
// CREATE OPERATOR FAMILY name USING index_method
//
// URL: https://www.postgresql.org/docs/18/sql-createopfamily.html

#[test]
fn create_operator_family_parses() {
    run_cases(&[r#"CREATE OPERATOR FAMILY my_family USING btree"#]);
}
