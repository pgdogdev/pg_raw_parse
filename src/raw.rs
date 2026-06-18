use crate::nodes::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

    // We need the raw binding to Node for tag checking, List and
    // MemoryContextData are both their own thing, Expr is just an alias for
    // Node
    assert_eq!(node_structs, &["Expr", "List", "MemoryContextData", "Node"]);
}
