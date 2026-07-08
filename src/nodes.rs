use crate::const_val::ConstValue;
use crate::make::{PgStr, Unique};
use crate::raw::{self, __IncompleteArrayField, List, Node, ValUnion};
use crate::{AsNodePtr, ConstructableNode, FromNodeMut};
use generativity::Id;
use std::fmt;
use std::ptr::NonNull;

pub use crate::raw::{A_Expr_Kind, BoolExprType, SortByDir, SortByNulls};

include!(concat!(env!("OUT_DIR"), "/nodes_raw.rs"));

impl Bitmapset {
    pub fn words(&self) -> &[bitmapword] {
        // SAFETY: words is always nwords long
        unsafe { self.words.as_slice(self.nwords as _) }
    }
}

impl RawStmt {
    pub(crate) fn new<N: AsNodePtr>(node: N) -> Self {
        Self {
            type_: raw::NodeTag_T_RawStmt,
            stmt: node.as_ptr(),
            stmt_location: -1,
            stmt_len: 0,
        }
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
        distinct_clause: [],
        into_clause: None,
        target_list: [
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
        from_clause: [
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
        where_clause: A_Expr(
            A_Expr {
                kind: AEXPR_OP,
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
        group_clause: [],
        group_distinct: false,
        having_clause: None,
        window_clause: [],
        values_lists: [],
        sort_clause: [],
        limit_offset: None,
        limit_count: None,
        limit_option: 0,
        locking_clause: [],
        with_clause: None,
        op: 0,
        all: false,
        larg: None,
        rarg: None,
        ..
    },
)"#;
    pretty_assertions::assert_eq!(expected, format!("{:#?}", stmt));
}
