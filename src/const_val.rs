#![allow(non_upper_case_globals)]

use crate::make::MemoryToken;
use crate::nodes;
use crate::raw::{NodeTag, ValUnion};
use std::ffi::c_int;
use std::mem::ManuallyDrop;
use std::str::FromStr;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ConstValue<'a> {
    Integer(c_int) = NodeTag::T_Integer,
    Float(&'a str) = NodeTag::T_Float,
    Boolean(bool) = NodeTag::T_Boolean,
    String(&'a str) = NodeTag::T_String,
    BitString(&'a str) = NodeTag::T_BitString,
    /// Either a node that isn't one of the ValUnion variants, or Postgres
    /// has added a new node that needs to be handled in this crate.
    Unrecognized(NodeTag::Type),
}

impl<'a> ConstValue<'a> {
    /// Turn the tagged union into a Rust enum. Any C string pointers are
    /// converted into &str, returning an empty string if they were NULL
    /// (though we never expect that to happen)
    pub(crate) fn from_raw(val: &'a ValUnion) -> Self {
        // SAFETY: We always check the tag before casting the union
        unsafe {
            match val.node.type_ {
                NodeTag::T_Integer => Self::Integer(val.ival.ival),
                NodeTag::T_Float => Self::Float(val.fval.fval().unwrap_or_default()),
                NodeTag::T_Boolean => Self::Boolean(val.boolval.boolval),
                NodeTag::T_String => Self::String(val.sval.sval().unwrap_or_default()),
                NodeTag::T_BitString => Self::BitString(val.bsval.bsval().unwrap_or_default()),
                tag => Self::Unrecognized(tag),
            }
        }
    }

    pub(crate) fn as_raw(&self, mem: MemoryToken<'_>) -> ValUnion {
        match self {
            Self::Integer(i) => ValUnion {
                ival: ManuallyDrop::new(nodes::Integer {
                    type_: NodeTag::T_Integer,
                    ival: *i,
                }),
            },
            Self::Float(f) => ValUnion {
                fval: ManuallyDrop::new(nodes::Float {
                    type_: NodeTag::T_Float,
                    fval: mem.copy_string(*f).into_ptr(),
                }),
            },
            Self::Boolean(b) => ValUnion {
                boolval: ManuallyDrop::new(nodes::Boolean {
                    type_: NodeTag::T_Boolean,
                    boolval: *b,
                }),
            },
            Self::String(s) => ValUnion {
                sval: ManuallyDrop::new(nodes::String {
                    type_: NodeTag::T_String,
                    sval: mem.copy_string(*s).into_ptr(),
                }),
            },
            Self::BitString(bs) => ValUnion {
                bsval: ManuallyDrop::new(nodes::BitString {
                    type_: NodeTag::T_BitString,
                    bsval: mem.copy_string(*bs).into_ptr(),
                }),
            },
            Self::Unrecognized(_) => panic!("as_raw called on unrecognized union type"),
        }
    }

    /// Fetches the value of this constant as the given numeric type.
    /// For integers smaller than c_int, this will be a simple integer cast.
    /// For "integer-looking strings" that are too big to fit in c_int, they
    /// will have been lexed as a float, which is actually a string.
    ///
    /// Returns None if this value is not a number, or if the value failed
    /// to parse into the given type.
    #[inline]
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
    #[inline]
    pub fn bool_value(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Fetches the value of this string constant
    ///
    /// Returns None if this value is not a string
    #[inline]
    pub fn string_value(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Fetches the value of this bitstring constant
    ///
    /// Returns None if this value is not a bistring
    #[inline]
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
    use crate::AsNodePtr;
    use crate::make::owned;

    #[test]
    fn test_integer_value() {
        let smallint = owned(|mem| mem.make_integer(1));
        let bigint = owned(|mem| mem.make_float(Some("1234567890")));
        let boolval = owned(|mem| mem.make_boolean(true));

        let smallunion = unsafe { &*(smallint.as_ptr()).cast() };
        let bigunion = unsafe { &*(bigint.as_ptr()).cast() };
        let boolunion = unsafe { &*(boolval.as_ptr()).cast() };

        let smallval = ConstValue::from_raw(smallunion);
        let bigval = ConstValue::from_raw(bigunion);
        let boolval = ConstValue::from_raw(boolunion);

        assert_eq!(Some(1), smallval.numeric_value::<i32>());
        assert_eq!(Some(1.0), smallval.numeric_value::<f64>());
        assert_eq!(Some(1234567890), bigval.numeric_value::<i64>());
        assert_eq!(None, boolval.numeric_value::<i32>());
    }
}
