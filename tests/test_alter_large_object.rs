mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     ALTER LARGE OBJECT
// Description: change the definition of a large object
// Syntax:
// ALTER LARGE OBJECT large_object_oid OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// URL: https://www.postgresql.org/docs/18/sql-alterlargeobject.html

#[test]
fn alter_large_object_parses() {
    run_cases(&[r#"ALTER LARGE OBJECT 12345 OWNER TO CURRENT_USER"#]);
}
