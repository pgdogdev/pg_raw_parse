#![cfg_attr(feature = "field_offset_assertions", feature(offset_of_enum))]
use std::{fmt, ops};

pub mod const_val;
mod deparse;
pub mod error;
pub mod list;
pub mod list_mut;
pub mod make;
mod mem;
pub mod node_enum;
mod node_ptr;
pub mod nodes;
pub mod normalize;
mod owned;
mod pg_error;
pub mod raw;
pub mod transform;
pub mod walk;

pub use crate::const_val::ConstValue;
pub use crate::deparse::{DeparseResult, deparse, deparse_stmts};
pub use crate::error::{Error, Result};
pub use crate::node_enum::{Node, NodeMut};
pub use crate::owned::Owned;

pub(crate) use node_ptr::{
    AsNodePtr, AsNodeRef, ConstructableNode, FromNodeMut, FromNodePtr, List,
};

pub fn parse(sql: &str) -> Result<ParseResult, error::Error> {
    Ok(ParseResult {
        tree: make::try_owned(|mem| mem.parse(sql))?,
    })
}

pub type StmtList = list::CastNodeList<nodes::RawStmt>;

pub struct ParseResult {
    tree: Owned<StmtList>,
}

impl ParseResult {
    /// Returns the list of raw statements that were parsed, discarding any
    /// warnings
    pub fn into_inner(self) -> Owned<StmtList> {
        self.tree
    }
}

impl ops::Deref for ParseResult {
    type Target = Owned<StmtList>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl fmt::Debug for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParseResult")
            .field("tree", &**self)
            .finish_non_exhaustive()
    }
}
