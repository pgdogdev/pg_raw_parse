#![allow(non_snake_case, clippy::tabs_in_doc_comments)]
use crate::raw::{__IncompleteArrayField, List, Node};

include!(concat!(env!("OUT_DIR"), "/nodes_raw.rs"));

impl Bitmapset {
    pub fn words(&self) -> &[bitmapword] {
        // SAFETY: words is always nwords long
        unsafe { self.words.as_slice(self.nwords as _) }
    }
}
