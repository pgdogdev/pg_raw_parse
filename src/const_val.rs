#![allow(non_upper_case_globals)]

use crate::raw::{
    NodeTag, NodeTag_T_BitString, NodeTag_T_Boolean, NodeTag_T_Float, NodeTag_T_Integer,
    NodeTag_T_String, ValUnion,
};
use std::ffi::c_int;
use std::str::FromStr;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ConstValue<'a> {
    Integer(c_int) = NodeTag_T_Integer,
    Float(&'a str) = NodeTag_T_Float,
    Boolean(bool) = NodeTag_T_Boolean,
    String(&'a str) = NodeTag_T_String,
    BitString(&'a str) = NodeTag_T_BitString,
    /// Either a node that isn't one of the ValUnion variants, or Postgres
    /// has added a new node that needs to be handled in this crate.
    Unrecognized(NodeTag),
}

impl<'a> ConstValue<'a> {
    /// Turn the tagged union into a Rust enum. Any C string pointers are
    /// converted into &str, returning an empty string if they were NULL
    /// (though we never expect that to happen)
    pub(crate) fn from_raw(val: &'a ValUnion) -> Self {
        // SAFETY: No matter what type this is, it's still a node
        let tag = unsafe { val.node.as_ref() }.type_;
        // SAFETY: We always check the tag before casting the union
        unsafe {
            match tag {
                NodeTag_T_Integer => Self::Integer(val.ival.as_ref().ival),
                NodeTag_T_Float => Self::Float(val.fval.as_ref().fval().unwrap_or_default()),
                NodeTag_T_Boolean => Self::Boolean(val.boolval.as_ref().boolval),
                NodeTag_T_String => Self::String(val.sval.as_ref().sval().unwrap_or_default()),
                NodeTag_T_BitString => {
                    Self::BitString(val.bsval.as_ref().bsval().unwrap_or_default())
                }
                _ => Self::Unrecognized(tag),
            }
        }
    }

    /// Fetches the value of this constant as the given numeric type.
    /// For integers smaller than c_int, this will be a simple integer cast.
    /// For "integer-looking strings" that are too big to fit in c_int, they
    /// will have been lexed as a float, which is actually a string.
    ///
    /// Returns None if this value is not a number, or if the value failed
    /// to parse into the given type.
    pub fn numeric_value<I: From<c_int> + FromStr>(&self) -> Option<I> {
        match self {
            Self::Integer(i) => Some((*i).into()),
            Self::Float(f) => f.parse().ok(),
            _ => None,
        }
    }

    /// Fetches the value of this boolean constant
    ///
    /// Returns None if this value is not a boolean
    pub fn bool_value(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Fetches the value of this string constant
    ///
    /// Returns None if this value is not a string
    pub fn string_value(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Fetches the value of this bitstring constant
    ///
    /// Returns None if this value is not a bistring
    pub fn bitstring_value(&self) -> Option<&str> {
        match self {
            Self::BitString(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes;

    #[test]
    fn test_integer_value() {
        let smallint = nodes::Integer {
            type_: NodeTag_T_Integer,
            ival: 1,
        };
        let bigint = nodes::Float {
            type_: NodeTag_T_Float,
            fval: c"1234567890".as_ptr().cast_mut(),
        };
        let boolval = nodes::Boolean {
            type_: NodeTag_T_Boolean,
            boolval: true,
        };

        let smallunion = unsafe { *(&raw const smallint).cast() };
        let bigunion = unsafe { *(&raw const bigint).cast() };
        let boolunion = unsafe { *(&raw const boolval).cast() };

        let smallval = ConstValue::from_raw(&smallunion);
        let bigval = ConstValue::from_raw(&bigunion);
        let boolval = ConstValue::from_raw(&boolunion);

        assert_eq!(Some(1), smallval.numeric_value::<i32>());
        assert_eq!(Some(1.0), smallval.numeric_value::<f64>());
        assert_eq!(Some(1234567890), bigval.numeric_value::<i64>());
        assert_eq!(None, boolval.numeric_value::<i32>());
    }
}
