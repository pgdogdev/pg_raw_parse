mod common;

use common::run_parse_debug_case as run_case;
use pg_raw_parse::{Node, parse};

// Command:     ALTER TEXT SEARCH CONFIGURATION
// Description: change the definition of a text search configuration
// Syntax:
// ALTER TEXT SEARCH CONFIGURATION name
//     ADD MAPPING FOR token_type [, ... ] WITH dictionary_name [, ... ]
// ALTER TEXT SEARCH CONFIGURATION name
//     ALTER MAPPING FOR token_type [, ... ] WITH dictionary_name [, ... ]
// ALTER TEXT SEARCH CONFIGURATION name
//     ALTER MAPPING REPLACE old_dictionary WITH new_dictionary
// ALTER TEXT SEARCH CONFIGURATION name
//     ALTER MAPPING FOR token_type [, ... ] REPLACE old_dictionary WITH new_dictionary
// ALTER TEXT SEARCH CONFIGURATION name
//     DROP MAPPING [ IF EXISTS ] FOR token_type [, ... ]
// ALTER TEXT SEARCH CONFIGURATION name RENAME TO new_name
// ALTER TEXT SEARCH CONFIGURATION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TEXT SEARCH CONFIGURATION name SET SCHEMA new_schema
//
// URL: https://www.postgresql.org/docs/18/sql-altertsconfig.html

#[test]
fn alter_text_search_configuration_add_mapping() {
    run_case(r#"ALTER TEXT SEARCH CONFIGURATION my_config ADD MAPPING FOR asciiword WITH simple"#);
}

#[test]
fn alter_text_search_configuration_alter_mapping_for_tokens() {
    run_case(
        r#"ALTER TEXT SEARCH CONFIGURATION my_config ALTER MAPPING FOR asciiword WITH english_stem"#,
    );
}

#[test]
fn alter_text_search_configuration_replace_mapping_dictionary() {
    run_case(
        r#"ALTER TEXT SEARCH CONFIGURATION my_config ALTER MAPPING REPLACE simple WITH english_stem"#,
    );
}

#[test]
fn alter_text_search_configuration_replace_mapping_for_tokens() {
    run_case(
        r#"ALTER TEXT SEARCH CONFIGURATION my_config ALTER MAPPING FOR asciiword REPLACE simple WITH english_stem"#,
    );
}

#[test]
fn alter_text_search_configuration_drop_mapping() {
    run_case(r#"ALTER TEXT SEARCH CONFIGURATION my_config DROP MAPPING IF EXISTS FOR asciiword"#);
}

#[test]
fn alter_text_search_configuration_rename() {
    run_case(r#"ALTER TEXT SEARCH CONFIGURATION my_config RENAME TO my_config_new"#);
}

#[test]
fn alter_text_search_configuration_owner() {
    run_case(r#"ALTER TEXT SEARCH CONFIGURATION my_config OWNER TO CURRENT_USER"#);
}

#[test]
fn alter_text_search_configuration_set_schema() {
    run_case(r#"ALTER TEXT SEARCH CONFIGURATION my_config SET SCHEMA public"#);
}

#[test]
fn alter_text_search_configuration_mapping_fields_are_node_lists() {
    let ast =
        parse(r#"ALTER TEXT SEARCH CONFIGURATION my_config ADD MAPPING FOR asciiword WITH simple"#)
            .unwrap();

    let Node::AlterTSConfigurationStmt(stmt) = ast.stmts().next().unwrap() else {
        panic!("expected AlterTSConfigurationStmt");
    };

    // tokentype() is a generic NodeList. For named token types, entries are
    // String nodes.
    let token_type = stmt.tokentype().first().expect("expected token type");
    assert_eq!(token_type.as_str(), Some("asciiword"));

    // Dictionaries are represented as a list of qualified-name lists. For the
    // unqualified dictionary `simple`, dicts() contains one nested NodeList,
    // and that nested list contains one String node.
    let dictionary_name = stmt
        .dicts()
        .first()
        .expect("expected dictionary")
        .expect_node_list();
    assert_eq!(
        dictionary_name.first().and_then(Node::as_str),
        Some("simple")
    );
}
