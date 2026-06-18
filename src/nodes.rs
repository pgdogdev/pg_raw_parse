#![allow(non_snake_case)]
use crate::raw::{__IncompleteArrayField, List, Node, ValUnion};

include!(concat!(env!("OUT_DIR"), "/nodes_raw.rs"));

impl Bitmapset {
    pub fn words(&self) -> &[bitmapword] {
        // SAFETY: words is always nwords long
        unsafe { self.words.as_slice(self.nwords as _) }
    }
}

pub type DistinctExpr = OpExpr;
pub type NullIfExpr = OpExpr;
