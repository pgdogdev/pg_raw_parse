#![allow(non_snake_case, clippy::tabs_in_doc_comments)]
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

impl fmt::Debug for A_Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("A_Const")
            .field("val", &self.val())
            .field("isnull", &self.isnull)
            .field("location", &self.location)
            .finish_non_exhaustive()
    }
}
