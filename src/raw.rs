//! Functions in this module should never be called unless they have been
//! manually wrapped *IN C* with `PG_TRY()` and `PG_CATCH()`. PG errors use
//! `longjmp`, and jumping over any Rust frames is undefined behavior.
#![allow(warnings)]

use crate::nodes::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Error {
    pub(crate) fn null() -> Self {
        Self {
            mem: std::ptr::null_mut(),
            error_data: std::ptr::null_mut(),
        }
    }
}

#[test]
fn test_raw_node_bindings_arent_generated() {
    use std::convert::identity as id;
    use syn::parse_quote;

    let bindings_source =
        std::fs::read_to_string(concat!(env!("OUT_DIR"), "/bindings.rs")).unwrap();
    let bindings = syn::parse_file(&bindings_source).unwrap();
    let mut node_structs = bindings
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(s) => Some(s),
            _ => None,
        })
        .filter(|s| {
            matches!(
                s.fields.iter().nth(0),
                Some(syn::Field {
                    ty,
                    ..
                }) if *ty == id::<syn::Type>(parse_quote!(NodeTag))
                    || *ty == id::<syn::Type>(parse_quote!(Expr))
            )
        })
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>();
    node_structs.sort();

    // These are the nodes that we either have special handling for, or have
    // explicitly blocklisted because they aren't parse nodes and handling
    // them would require extra code
    assert_eq!(
        node_structs,
        &[
            "Const",             // A_Const is the parsed version
            "Expr",              // Abstract type
            "JsonTablePath",     // JsonTablePathSpec is the parsed version
            "JsonTablePlan",     // JsonTablePlanSpec is the parsed version
            "List",              // list::NodeList
            "MemoryContextData", // mem::MemoryContext
            "Node",              // node_enum::Node
            "RelabelType",       // Implicit coercion, never parsed
            "Var",               // Used during optimization, not parsing
        ],
    );
}
