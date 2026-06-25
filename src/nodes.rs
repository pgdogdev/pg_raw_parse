#![allow(non_snake_case)]
use crate::const_val::ConstValue;
use crate::raw::{__IncompleteArrayField, List, Node, ValUnion};
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/nodes_raw.rs"));

impl Bitmapset {
    pub fn words(&self) -> &[bitmapword] {
        // SAFETY: words is always nwords long
        unsafe { self.words.as_slice(self.nwords as _) }
    }
}

struct __DebugIterator<F>(F);

impl<F, I> fmt::Debug for __DebugIterator<F>
where
    F: Fn() -> I,
    I: IntoIterator,
    I::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.0()).finish()
    }
}

#[test]
// If this test begins failing due to a change in the standard library's
// formatter, update this test as long as the output continues to look
// reasonable
fn test_debug_output() {
    let result = crate::parse("SELECT * FROM users WHERE id = 1").unwrap();
    let stmt = result.stmts().next().unwrap();
    let expected = r#"SelectStmt(
    SelectStmt {
        distinctClause: [],
        intoClause: None,
        targetList: [
            ResTarget {
                name: None,
                indirection: [],
                val: ColumnRef(
                    ColumnRef {
                        fields: [
                            A_Star(
                                A_Star { .. },
                            ),
                        ],
                        ..
                    },
                ),
                ..
            },
        ],
        fromClause: [
            RangeVar(
                RangeVar {
                    catalogname: None,
                    schemaname: None,
                    relname: Some(
                        "users",
                    ),
                    inh: true,
                    relpersistence: 112,
                    alias: None,
                    ..
                },
            ),
        ],
        whereClause: A_Expr(
            A_Expr {
                kind: 0,
                name: [
                    String(
                        String {
                            sval: Some(
                                "=",
                            ),
                            ..
                        },
                    ),
                ],
                lexpr: ColumnRef(
                    ColumnRef {
                        fields: [
                            String(
                                String {
                                    sval: Some(
                                        "id",
                                    ),
                                    ..
                                },
                            ),
                        ],
                        ..
                    },
                ),
                rexpr: A_Const(
                    A_Const {
                        val: Some(
                            Integer(
                                1,
                            ),
                        ),
                        isnull: false,
                        ..
                    },
                ),
                ..
            },
        ),
        groupClause: [],
        groupDistinct: false,
        havingClause: None,
        windowClause: [],
        valuesLists: [],
        sortClause: [],
        limitOffset: None,
        limitCount: None,
        limitOption: 0,
        lockingClause: [],
        withClause: None,
        op: 0,
        all: false,
        larg: None,
        rarg: None,
        ..
    },
)"#;
    assert_eq!(expected, format!("{:#?}", stmt));
}
