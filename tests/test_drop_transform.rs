mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn drop_transform_parses() {
    run_cases(&["DROP TRANSFORM IF EXISTS FOR integer LANGUAGE plpython3u"]);
}
