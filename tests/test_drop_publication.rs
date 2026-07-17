mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP PUBLICATION
// Description: remove a publication
// Syntax:
// DROP PUBLICATION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droppublication.html

#[test]
fn drop_publication_parses() {
    run_cases(&[
        r#"DROP PUBLICATION pub"#,
        r#"DROP PUBLICATION IF EXISTS pub, pub_old CASCADE"#,
        r#"DROP PUBLICATION IF EXISTS pub RESTRICT"#,
    ]);
}
