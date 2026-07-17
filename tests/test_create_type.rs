mod common;

use common::run_parse_debug_cases as run_cases;

#[test]
fn create_type_parses() {
    run_cases(&[
        r#"CREATE TYPE address AS (street text COLLATE "C", zip integer)"#,
        r#"CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy')"#,
        r#"CREATE TYPE inventory_item AS RANGE (SUBTYPE = integer, SUBTYPE_OPCLASS = int4_ops, COLLATION = "C", CANONICAL = my_canonical, SUBTYPE_DIFF = my_diff, MULTIRANGE_TYPE_NAME = inventory_item_multirange)"#,
        r#"CREATE TYPE shell_type"#,
        r#"CREATE TYPE my_base_type (INPUT = my_input, OUTPUT = my_output, RECEIVE = my_receive, SEND = my_send, INTERNALLENGTH = variable, PASSEDBYVALUE, ALIGNMENT = int4, STORAGE = extended, LIKE = integer, CATEGORY = 'U', PREFERRED = true, DEFAULT = '0', ELEMENT = integer, DELIMITER = ',', COLLATABLE = true)"#,
    ]);
}
