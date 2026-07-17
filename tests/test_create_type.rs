mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     CREATE TYPE
// Description: define a new data type
// Syntax:
// CREATE TYPE name AS
//     ( [ attribute_name data_type [ COLLATE collation ] [, ... ] ] )
//
// CREATE TYPE name AS ENUM
//     ( [ 'label' [, ... ] ] )
//
// CREATE TYPE name AS RANGE (
//     SUBTYPE = subtype
//     [ , SUBTYPE_OPCLASS = subtype_operator_class ]
//     [ , COLLATION = collation ]
//     [ , CANONICAL = canonical_function ]
//     [ , SUBTYPE_DIFF = subtype_diff_function ]
//     [ , MULTIRANGE_TYPE_NAME = multirange_type_name ]
// )
//
// CREATE TYPE name (
//     INPUT = input_function,
//     OUTPUT = output_function
//     [ , RECEIVE = receive_function ]
//     [ , SEND = send_function ]
//     [ , TYPMOD_IN = type_modifier_input_function ]
//     [ , TYPMOD_OUT = type_modifier_output_function ]
//     [ , ANALYZE = analyze_function ]
//     [ , SUBSCRIPT = subscript_function ]
//     [ , INTERNALLENGTH = { internallength | VARIABLE } ]
//     [ , PASSEDBYVALUE ]
//     [ , ALIGNMENT = alignment ]
//     [ , STORAGE = storage ]
//     [ , LIKE = like_type ]
//     [ , CATEGORY = category ]
//     [ , PREFERRED = preferred ]
//     [ , DEFAULT = default ]
//     [ , ELEMENT = element ]
//     [ , DELIMITER = delimiter ]
//     [ , COLLATABLE = collatable ]
// )
//
// CREATE TYPE name
//
// URL: https://www.postgresql.org/docs/18/sql-createtype.html

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
