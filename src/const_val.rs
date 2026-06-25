#![allow(non_upper_case_globals)]

use crate::raw::{
    NodeTag, NodeTag_T_Boolean, NodeTag_T_Float, NodeTag_T_Integer, NodeTag_T_String, ValUnion,
};
use std::ffi::c_int;
use std::fmt;
use std::str::FromStr;

pub struct ConstValue<'a>(pub(crate) &'a ValUnion);

impl ConstValue<'_> {
    fn tag(&self) -> NodeTag {
        // SAFETY: No matter what version of the union this is, it's always a Node.
        unsafe { self.0.node.as_ref() }.type_
    }

    /// Fetches the value of this constant as the given numeric type.
    /// For integers smaller than c_int, this will be a simple integer cast.
    /// For "integer-looking strings" that are too big to fit in c_int, they
    /// will have been lexed as a float, which is actually a string.
    ///
    /// Returns None if this value is not a number, or if the value failed
    /// to parse into the given type.
    pub fn numeric_value<I: From<c_int> + FromStr>(&self) -> Option<I> {
        // SAFETY: We're matching the tag
        match self.tag() {
            NodeTag_T_Integer => Some(unsafe { self.0.ival.as_ref() }.ival.into()),
            NodeTag_T_Float => unsafe { self.0.fval.as_ref() }.fval()?.parse().ok(),
            _ => None,
        }
    }

    /// Fetches the value of this constant as a boolean
    ///
    /// Returns None if this value is not a bool
    pub fn bool_value(&self) -> Option<bool> {
        // SAFETY: We're matching the tag
        match self.tag() {
            NodeTag_T_Boolean => Some(unsafe { self.0.boolval.as_ref() }.boolval),
            _ => None,
        }
    }

    /// Fetches the value of this constant as a string
    ///
    /// Returns None if this value is not a string
    pub fn str_value(&self) -> Option<&str> {
        // SAFETY: We're matching the tag
        match self.tag() {
            NodeTag_T_String => unsafe { self.0.sval.as_ref() }.sval(),
            _ => None,
        }
    }
}

impl fmt::Debug for ConstValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("ConstValue")
            // SAFETY: We're checking the tag
            .field(unsafe {
                match self.tag() {
                    NodeTag_T_Integer => self.0.ival.as_ref(),
                    NodeTag_T_Float => self.0.fval.as_ref(),
                    NodeTag_T_Boolean => self.0.boolval.as_ref(),
                    NodeTag_T_String => self.0.sval.as_ref(),
                    _ => return Ok(()),
                }
            })
            .finish()
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

        let smallval = ConstValue(&unsafe { *(&raw const smallint).cast() });
        let bigval = ConstValue(&unsafe { *(&raw const bigint).cast() });
        let boolval = ConstValue(&unsafe { *(&raw const boolval).cast() });

        assert_eq!(Some(1), smallval.numeric_value::<i32>());
        assert_eq!(Some(1.0), smallval.numeric_value::<f64>());
        assert_eq!(Some(1234567890), bigval.numeric_value::<i64>());
        assert_eq!(None, boolval.numeric_value::<i32>());
    }
}
