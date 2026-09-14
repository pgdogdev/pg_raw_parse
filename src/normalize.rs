use std::{collections::HashSet, ptr};

use crate::list::NodeList;
use crate::list_mut::NodeListMut;
use crate::transform::{self, Assignable, Transform};
use crate::{ConstValue, DeparseResult, Node, NodeMut, Owned, deparse, make, nodes, parse};

pub fn normalize(query: &nodes::RawStmt) -> Owned<nodes::RawStmt> {
    make::owned(|mem| {
        let mut copy = mem.make_unique(query);
        Normalizer {
            mem,
            param_count: 0,
            ordinals: HashSet::new(),
        }
        .transform_raw_stmt(copy.as_mut());
        copy
    })
}

struct Normalizer<'mem> {
    mem: make::MemoryToken<'mem>,
    param_count: i32,
    // Compare node identities only: the copied AST owns these constants for
    // the entire traversal. Equal integer values elsewhere remain expressions.
    ordinals: HashSet<*const nodes::A_Const>,
}

impl<'mem> Normalizer<'mem> {
    fn preserve_ordinal(&mut self, node: Node<'_>) {
        if let Node::A_Const(value) = node
            && matches!(value.val(), Some(ConstValue::Integer(_)))
        {
            self.ordinals.insert(ptr::from_ref(value));
        }
    }

    // GROUP BY GROUPING SETS ((1, 2), ROLLUP(1)) refers to output columns,
    // so the integers must survive even inside nested grouping lists.
    // GROUP BY ROW(1, 2) instead becomes GROUP BY ROW($1, $2).
    fn preserve_group_ordinals(&mut self, node: Node<'_>) {
        match node {
            Node::GroupingSet(group) => {
                for node in group.content() {
                    self.preserve_group_ordinals(node);
                }
            }
            Node::RowExpr(row) if row.row_format == nodes::CoercionForm::COERCE_IMPLICIT_CAST => {
                // GROUP BY (1, 2) is a grouping list; ROW(1, 2) is an expression.
                for node in row.args() {
                    self.preserve_group_ordinals(node);
                }
            }
            _ => self.preserve_ordinal(node),
        }
    }

    // These SQL constructs mix expression arguments with syntax constants.
    // Copy only the expression argument so it can be replaced independently.
    fn normalize_first_arg(&mut self, mut args: NodeListMut<'mem, '_, NodeList>) {
        if let Some(arg) = args.get(0) {
            let mut arg = self.mem.make_unique(arg);
            self.transform_node(Assignable::new(&mut arg));
            args.set(0, arg);
        }
    }
}

impl<'mem> Transform<'mem> for Normalizer<'mem> {
    fn transform_node<'mutref>(&mut self, mut node: Assignable<'mem, 'mutref>) {
        match &mut *node {
            NodeMut::A_Const(value) if self.ordinals.contains(&ptr::from_ref(&**value)) => {}
            NodeMut::A_Const(_) => {
                self.param_count += 1;
                node.replace(self.mem.make_param_ref(self.param_count).uncast());
            }
            NodeMut::ParamRef(p) => {
                self.param_count += 1;
                p.set_number(self.param_count);
            }
            _ => transform::transform_node(node.into_inner(), self),
        }
    }

    // SQL:        SELECT 42 GROUP BY 1 ORDER BY 1
    // Normalized: SELECT $1 GROUP BY 1 ORDER BY 1
    // The ordinal 1 identifies the first output column; $2 would be a constant
    // expression instead. Expressions like ORDER BY a + 1 still become a + $n.
    // Likewise, sum(a ORDER BY 1) and OVER (ORDER BY 1) use constant expressions,
    // not output-column positions, so those integers must become parameters.
    fn transform_select_stmt<'mutref>(&mut self, node: nodes::SelectStmtMut<'mem, 'mutref>) {
        for group in node.group_clause() {
            self.preserve_group_ordinals(group);
        }
        for sort in node.sort_clause() {
            self.preserve_ordinal(sort.node());
        }
        // Use the generated traversal to retain its parameter numbering order.
        // Only SELECT's sort clause has ordinals: aggregate and window ORDER BY
        // integers are ordinary expressions and must still be normalized.
        transform::transform_select_stmt(node, self);
    }

    // Transaction options are syntax constants, not expressions. The native
    // deparser reads their values as A_Const nodes; replacing an isolation
    // level with ParamRef makes it dereference an invalid string pointer.
    // SQL: BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY
    // Keep both options unchanged. REPEATABLE READ is stored as a string
    // constant and READ ONLY as integer 1; neither can become $1 or $2.
    fn transform_transaction_stmt<'mutref>(
        &mut self,
        _node: nodes::TransactionStmtMut<'mem, 'mutref>,
    ) {
    }

    // SET arguments also require literal values, including SET TRANSACTION.
    // Override the typed visitor so this also covers SET nested in ALTER ROLE,
    // ALTER DATABASE and function options.
    // SQL: SET TRANSACTION ISOLATION LEVEL SERIALIZABLE
    // Keep SERIALIZABLE intact for the same reason as BEGIN's isolation level.
    // SET TIME ZONE 'UTC' and ALTER ROLE app SET statement_timeout = 1000
    // also retain their setting values, which are not parameter expressions.
    fn transform_variable_set_stmt<'mutref>(
        &mut self,
        _node: nodes::VariableSetStmtMut<'mem, 'mutref>,
    ) {
    }

    // Type modifiers encode precision, length and interval units. They are
    // not expression parameters, even when stored as A_Const nodes.
    // SQL:        SELECT 'abc'::varchar(10), INTERVAL '1.234' SECOND(2)
    // Normalized: SELECT $1::varchar(10), $2::interval SECOND(2)
    // varchar($n) is invalid syntax. Replacing interval's internal unit/precision
    // constants can also make the deparser read the wrong units or precision.
    fn transform_type_name<'mutref>(&mut self, _node: nodes::TypeNameMut<'mem, 'mutref>) {}

    // JSON_TABLE paths must remain string constants; the deparser directly
    // reads their A_Const string values.
    // SQL: SELECT * FROM JSON_TABLE('[1]'::jsonb, '$[*]'
    //                               COLUMNS (value int PATH '$')) AS jt
    // Only '[1]' becomes $1; '$[*]' and '$' must remain string constants.
    // Replacing a path with ParamRef makes the deparser read an invalid pointer.
    fn transform_json_table_path_spec<'mutref>(
        &mut self,
        _node: nodes::JsonTablePathSpecMut<'mem, 'mutref>,
    ) {
    }

    // CYCLE mark/default values use the AexprConst grammar, not expressions.
    // SQL: WITH RECURSIVE t(id) AS (SELECT 1)
    //      CYCLE id SET cycle TO 'yes' DEFAULT 'no' USING path SELECT * FROM t
    // Normalize SELECT 1 to SELECT $1, but keep 'yes' and 'no': parameters in
    // those positions are rejected by the parser/deparser.
    fn transform_cte_cycle_clause<'mutref>(
        &mut self,
        _node: nodes::CTECycleClauseMut<'mem, 'mutref>,
    ) {
    }

    fn transform_xml_expr<'mutref>(&mut self, mut node: nodes::XmlExprMut<'mem, 'mutref>) {
        if node.op == nodes::XmlExprOp::IS_XMLROOT {
            // VERSION and STANDALONE are syntax constants, unlike the XML value.
            // SQL:        XMLROOT('<a/>'::xml, VERSION '1.0', STANDALONE YES)
            // Normalized: XMLROOT($1::xml, VERSION '1.0', STANDALONE YES)
            // The deparser reads VERSION's null flag and STANDALONE's integer
            // flag directly; replacing them can silently change the options.
            self.normalize_first_arg(node.args_mut());
        } else {
            transform::transform_xml_expr(node, self);
        }
    }

    fn transform_func_call<'mutref>(&mut self, mut node: nodes::FuncCallMut<'mem, 'mutref>) {
        let mut names = node.funcname().iter().filter_map(Node::as_str);
        let normalization_syntax = node.funcformat == nodes::CoercionForm::COERCE_SQL_SYNTAX
            && names.next() == Some("pg_catalog")
            && matches!(names.next(), Some("normalize" | "is_normalized"))
            && names.next().is_none();
        if normalization_syntax {
            // NFC/NFD/NFKC/NFKD is a keyword stored as an A_Const argument.
            // NORMALIZE('hello', NFC) becomes NORMALIZE($1, NFC), and
            // 'hello' IS NFC NORMALIZED becomes $1 IS NFC NORMALIZED.
            // Turning NFC into $2 breaks the deparser's constant-value read.
            // An ordinary call pg_catalog.normalize('hello', 'NFC') instead
            // becomes pg_catalog.normalize($1, $2), since both are expressions.
            self.normalize_first_arg(node.args_mut());
        } else {
            transform::transform_func_call(node, self);
        }
    }
}

pub fn normalize_str(query: &str) -> crate::Result<DeparseResult> {
    let tree = parse(query)?;
    if let Some(stmt) = tree.first() {
        deparse(&*normalize(stmt))
    } else {
        deparse(Node::None)
    }
}

#[test]
fn test_normalize_does_the_thing() {
    let normalized = normalize_str("SELECT * FROM users WHERE id = 1").unwrap();
    assert_eq!(normalized.as_str(), "SELECT * FROM users WHERE id = $1");

    let normalized = normalize_str("SELECT * FROM users WHERE id = 1 AND name = $1").unwrap();
    assert_eq!(
        normalized.as_str(),
        "SELECT * FROM users WHERE id = $1 AND name = $2"
    );

    assert!(normalize_str("").is_err());
}

#[test]
fn test_normalize_transaction_options() {
    for query in [
        "BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY",
        "BEGIN ISOLATION LEVEL READ UNCOMMITTED READ WRITE",
        "START TRANSACTION ISOLATION LEVEL SERIALIZABLE READ ONLY DEFERRABLE",
        "BEGIN READ WRITE NOT DEFERRABLE",
        "SET TRANSACTION ISOLATION LEVEL READ COMMITTED READ ONLY",
        "SET SESSION CHARACTERISTICS AS TRANSACTION ISOLATION LEVEL SERIALIZABLE",
    ] {
        let ast = parse(query).expect("valid transaction options");
        let stmt = ast.first().expect("one statement");
        let original = deparse(stmt).expect("original statement deparses");
        let normalized = normalize_str(query).expect("normalized statement deparses");
        assert_eq!(normalized.as_str(), original.as_str(), "{query}");
    }
}

#[test]
fn test_normalize_set_arguments() {
    for query in [
        "SET client_encoding = 'UTF8'",
        "SET client_min_messages TO WARNING",
        "SET TIME ZONE 'UTC'",
        "SET statement_timeout = 1000",
        "ALTER ROLE postgres SET default_transaction_isolation TO 'repeatable read'",
        "ALTER DATABASE postgres SET statement_timeout = 1000",
        "CREATE FUNCTION f() RETURNS int LANGUAGE SQL SET statement_timeout = 1000 AS 'SELECT 1'",
    ] {
        let ast = parse(query).expect("valid SET arguments");
        let stmt = ast.first().expect("one statement");
        let original = deparse(stmt).expect("original statement deparses");
        let normalized = normalize_str(query).expect("normalized statement deparses");
        assert_eq!(normalized.as_str(), original.as_str(), "{query}");
    }
}

#[test]
fn test_normalize_query_expressions() {
    for (query, expected) in [
        ("SELECT 42, 'hello', $7", "SELECT $1, $2, $3"),
        ("INSERT INTO t VALUES (42)", "INSERT INTO t VALUES ($1)"),
        (
            "UPDATE t SET id = 42 WHERE id = 1",
            "UPDATE t SET id = $1 WHERE id = $2",
        ),
        ("DELETE FROM t WHERE id = 42", "DELETE FROM t WHERE id = $1"),
        ("EXPLAIN SELECT 42", "EXPLAIN SELECT $1"),
    ] {
        let ast = parse(query).expect("valid query");
        let stmt = ast.first().expect("one statement");
        let original = deparse(stmt).expect("original statement deparses");
        let normalized = normalize(stmt);
        assert_eq!(
            deparse(&*normalized)
                .expect("normalized query deparses")
                .as_str(),
            expected
        );
        assert_eq!(
            deparse(stmt)
                .expect("original query still deparses")
                .as_str(),
            original.as_str()
        );
    }
}
