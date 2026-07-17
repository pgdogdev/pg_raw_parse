mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_publication_parses() {
    run_cases(&[
        r#"DROP PUBLICATION pub"#,
        r#"DROP PUBLICATION IF EXISTS pub, pub_old CASCADE"#,
        r#"DROP PUBLICATION IF EXISTS pub RESTRICT"#,
    ]);
}
