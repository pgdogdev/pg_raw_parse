mod common;

use common::run_parse_debug_cases as run_cases;

// Command:     DROP ACCESS METHOD
// Description: remove an access method
// Syntax:
// DROP ACCESS METHOD [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-drop-access-method.html
//
// Command:     DROP AGGREGATE
// Description: remove an aggregate function
// Syntax:
// DROP AGGREGATE [ IF EXISTS ] name ( aggregate_signature ) [, ...] [ CASCADE | RESTRICT ]
//
// where aggregate_signature is:
//
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropaggregate.html
//
// Command:     DROP CAST
// Description: remove a cast
// Syntax:
// DROP CAST [ IF EXISTS ] (source_type AS target_type) [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropcast.html
//
// Command:     DROP COLLATION
// Description: remove a collation
// Syntax:
// DROP COLLATION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropcollation.html
//
// Command:     DROP CONVERSION
// Description: remove a conversion
// Syntax:
// DROP CONVERSION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropconversion.html
//
// Command:     DROP DATABASE
// Description: remove a database
// Syntax:
// DROP DATABASE [ IF EXISTS ] name [ [ WITH ] ( option [, ...] ) ]
//
// where option can be:
//
//     FORCE
//
// URL: https://www.postgresql.org/docs/18/sql-dropdatabase.html
//
// Command:     DROP DOMAIN
// Description: remove a domain
// Syntax:
// DROP DOMAIN [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropdomain.html
//
// Command:     DROP EVENT TRIGGER
// Description: remove an event trigger
// Syntax:
// DROP EVENT TRIGGER [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropeventtrigger.html
//
// Command:     DROP EXTENSION
// Description: remove an extension
// Syntax:
// DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropextension.html
//
// Command:     DROP FOREIGN DATA WRAPPER
// Description: remove a foreign-data wrapper
// Syntax:
// DROP FOREIGN DATA WRAPPER [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropforeigndatawrapper.html
//
// Command:     DROP FOREIGN TABLE
// Description: remove a foreign table
// Syntax:
// DROP FOREIGN TABLE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropforeigntable.html
//
// Command:     DROP FUNCTION
// Description: remove a function
// Syntax:
// DROP FUNCTION [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropfunction.html
//
// Command:     DROP GROUP
// Description: remove a database role
// Syntax:
// DROP GROUP [ IF EXISTS ] name [, ...]
//
// URL: https://www.postgresql.org/docs/18/sql-dropgroup.html
//
// Command:     DROP INDEX
// Description: remove an index
// Syntax:
// DROP INDEX [ CONCURRENTLY ] [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropindex.html
//
// Command:     DROP LANGUAGE
// Description: remove a procedural language
// Syntax:
// DROP [ PROCEDURAL ] LANGUAGE [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droplanguage.html
//
// Command:     DROP MATERIALIZED VIEW
// Description: remove a materialized view
// Syntax:
// DROP MATERIALIZED VIEW [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropmaterializedview.html
//
// Command:     DROP OPERATOR
// Description: remove an operator
// Syntax:
// DROP OPERATOR [ IF EXISTS ] name ( { left_type | NONE } , right_type ) [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropoperator.html
//
// Command:     DROP OPERATOR CLASS
// Description: remove an operator class
// Syntax:
// DROP OPERATOR CLASS [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropopclass.html
//
// Command:     DROP OPERATOR FAMILY
// Description: remove an operator family
// Syntax:
// DROP OPERATOR FAMILY [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropopfamily.html
//
// Command:     DROP OWNED
// Description: remove database objects owned by a database role
// Syntax:
// DROP OWNED BY { name | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-drop-owned.html
//
// Command:     DROP POLICY
// Description: remove a row-level security policy from a table
// Syntax:
// DROP POLICY [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droppolicy.html
//
// Command:     DROP PROCEDURE
// Description: remove a procedure
// Syntax:
// DROP PROCEDURE [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropprocedure.html
//
// Command:     DROP PUBLICATION
// Description: remove a publication
// Syntax:
// DROP PUBLICATION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droppublication.html
//
// Command:     DROP ROLE
// Description: remove a database role
// Syntax:
// DROP ROLE [ IF EXISTS ] name [, ...]
//
// URL: https://www.postgresql.org/docs/18/sql-droprole.html
//
// Command:     DROP ROUTINE
// Description: remove a routine
// Syntax:
// DROP ROUTINE [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droproutine.html
//
// Command:     DROP RULE
// Description: remove a rewrite rule
// Syntax:
// DROP RULE [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droprule.html
//
// Command:     DROP SCHEMA
// Description: remove a schema
// Syntax:
// DROP SCHEMA [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropschema.html
//
// Command:     DROP SEQUENCE
// Description: remove a sequence
// Syntax:
// DROP SEQUENCE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropsequence.html
//
// Command:     DROP SERVER
// Description: remove a foreign server descriptor
// Syntax:
// DROP SERVER [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropserver.html
//
// Command:     DROP STATISTICS
// Description: remove extended statistics
// Syntax:
// DROP STATISTICS [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropstatistics.html
//
// Command:     DROP SUBSCRIPTION
// Description: remove a subscription
// Syntax:
// DROP SUBSCRIPTION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropsubscription.html
//
// Command:     DROP TABLE
// Description: remove a table
// Syntax:
// DROP TABLE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptable.html
//
// Command:     DROP TABLESPACE
// Description: remove a tablespace
// Syntax:
// DROP TABLESPACE [ IF EXISTS ] name
//
// URL: https://www.postgresql.org/docs/18/sql-droptablespace.html
//
// Command:     DROP TEXT SEARCH CONFIGURATION
// Description: remove a text search configuration
// Syntax:
// DROP TEXT SEARCH CONFIGURATION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptsconfig.html
//
// Command:     DROP TEXT SEARCH DICTIONARY
// Description: remove a text search dictionary
// Syntax:
// DROP TEXT SEARCH DICTIONARY [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptsdictionary.html
//
// Command:     DROP TEXT SEARCH PARSER
// Description: remove a text search parser
// Syntax:
// DROP TEXT SEARCH PARSER [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptsparser.html
//
// Command:     DROP TEXT SEARCH TEMPLATE
// Description: remove a text search template
// Syntax:
// DROP TEXT SEARCH TEMPLATE [ IF EXISTS ] name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptstemplate.html
//
// Command:     DROP TRANSFORM
// Description: remove a transform
// Syntax:
// DROP TRANSFORM [ IF EXISTS ] FOR type_name LANGUAGE lang_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptransform.html
//
// Command:     DROP TRIGGER
// Description: remove a trigger
// Syntax:
// DROP TRIGGER [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptrigger.html
//
// Command:     DROP TYPE
// Description: remove a data type
// Syntax:
// DROP TYPE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-droptype.html
//
// Command:     DROP USER
// Description: remove a database role
// Syntax:
// DROP USER [ IF EXISTS ] name [, ...]
//
// URL: https://www.postgresql.org/docs/18/sql-dropuser.html
//
// Command:     DROP USER MAPPING
// Description: remove a user mapping for a foreign server
// Syntax:
// DROP USER MAPPING [ IF EXISTS ] FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | PUBLIC } SERVER server_name
//
// URL: https://www.postgresql.org/docs/18/sql-dropusermapping.html
//
// Command:     DROP VIEW
// Description: remove a view
// Syntax:
// DROP VIEW [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// URL: https://www.postgresql.org/docs/18/sql-dropview.html

#[test]
fn drop_parses() {
    run_cases(&[
        r#"DROP TABLE IF EXISTS my_table"#,
        r#"DROP TABLE IF EXISTS my_table, other_table CASCADE"#,
    ]);
}
