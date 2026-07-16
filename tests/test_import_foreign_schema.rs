mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn import_foreign_schema_parses() {
    run_cases(&[
        "IMPORT FOREIGN SCHEMA remote_schema LIMIT TO (remote_table) FROM SERVER my_server INTO public",
    ]);
}
