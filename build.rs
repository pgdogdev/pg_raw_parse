use glob::glob;
use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use syn::{
    parse_quote,
    spanned::Spanned,
    visit::{self, Visit},
};

fn main() {
    let build_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR always present in build scripts"));
    let c_dir = build_dir.join("libpg_query");
    println!("cargo:rerun-if-changed=libpg_query");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=pg_raw_parse");

    // Bindgen args that are needed both for the C bindings and the node enum
    // codegen
    let bindgen = bindgen::builder()
        .header("wrapper.h")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(
            c_dir
                .join("src/postgres/include/nodes/parsenodes.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_file(
            c_dir
                .join("src/postgres/include/nodes/primnodes.h")
                .to_str()
                .unwrap(),
        )
        .derive_copy(false)
        .clang_arg(format!("-I{}", build_dir.join("include").display()))
        .clang_arg(format!("-I{}", c_dir.display()))
        .clang_arg(format!(
            "-I{}",
            c_dir.join("src/postgres/include").display()
        ));

    let mut node_bindings = String::new();
    bindgen
        .clone()
        // This is another name for Node
        .blocklist_type("Expr")
        // This is yet another name for Node
        .blocklist_type("JsonTablePlan")
        // Yes, we want doc comments
        .clang_arg("-fparse-all-comments")
        .derive_debug(false)
        .generate()
        .unwrap()
        // SAFETY: YOLO
        .write(Box::new(unsafe { node_bindings.as_mut_vec() }))
        .unwrap();
    let node_structs =
        generate_node_structs(&node_bindings, &out_dir.join("nodes_raw.rs")).unwrap();
    generate_node_enum(&node_structs, &out_dir.join("node_enum_raw.rs")).unwrap();

    let mut bindgen = bindgen
        .allowlist_item("Node")
        .allowlist_item("MemoryContext")
        .allowlist_item("pg_query_init")
        .allowlist_item("AllocSetContextCreateInternal")
        .allowlist_item("ALLOCSET_DEFAULT_MINSIZE")
        .allowlist_item("ALLOCSET_DEFAULT_INITSIZE")
        .allowlist_item("ALLOCSET_DEFAULT_MAXSIZE")
        .allowlist_item("get_top_memory_context")
        .allowlist_item("MemoryContextSwitchTo")
        .allowlist_item("MemoryContextDelete")
        .allowlist_item("PgQueryError")
        .allowlist_item("pg_query_free_error")
        .allowlist_item("pg_query_raw_parse")
        .allowlist_item("PgQueryParseMode")
        .allowlist_item("wrapped_raw_expression_tree_walker_impl")
        .override_abi(
            bindgen::Abi::CUnwind,
            "wrapped_raw_expression_tree_walker_impl",
        )
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"));
    for struct_name in &node_structs {
        bindgen = bindgen.blocklist_item(struct_name.to_string());
    }
    bindgen
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();

    let mut build = cc::Build::new();
    build
        .files(glob("libpg_query/src/*.c").unwrap().map(Result::unwrap))
        .files(
            glob("libpg_query/src/postgres/*.c")
                .unwrap()
                .map(Result::unwrap),
        )
        .file(out_dir.join("wrap_static_fns.c"))
        .file(build_dir.join("copy_pg_error.c"))
        // Unfortunately, the linker expects protobuf functions to be present
        // even if we're never using them
        .file(c_dir.join("vendor/protobuf-c/protobuf-c.c"))
        .file(c_dir.join("vendor/xxhash/xxhash.c"))
        .file(c_dir.join("protobuf/pg_query.pb-c.c"))
        .include(&*c_dir)
        .include(c_dir.join("vendor"))
        .include(c_dir.join("src/postgres/include"))
        .include(c_dir.join("src/include"))
        .include(build_dir)
        .include(build_dir.join("include"))
        .warnings(false)
        .compile("pg_raw_parse");
}

/// Generates the structs for each node and writes them to the given path.
/// Returns a list of the struct names generated
fn generate_node_structs(
    bindings: &str,
    path: &Path,
) -> Result<Vec<syn::Ident>, Box<dyn std::error::Error>> {
    let file = syn::parse_file(bindings)?;
    let mut out_file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: Vec::new(),
    };

    // We're relying on missing imports triggering an error to tell us about
    // any fields that need special handling, so we don't want to glob import
    // the C bindings. But any type aliases to primitives are fine
    let type_aliases_to_import = file.items.iter().filter_map(|item| match item {
        syn::Item::Type(t) if is_primitive_alias(t) => Some(&t.ident),
        _ => None,
    });
    out_file.items.push(parse_quote! {
        #[allow(unused)]
        use crate::raw::{#(#type_aliases_to_import,)*};
    });

    let node_structs = file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(s) => Some(s),
            _ => None,
        })
        .filter(|s| {
            // We don't want a Node enum variant for Node
            s.ident != "Node" &&
            // List is its own thing, not a type of Node
            s.ident != "List" &&
            matches!(
                s.fields.iter().next(),
                Some(syn::Field {
                    ty: typ,
                    ..
                }) if *typ == ty(parse_quote!(NodeTag))
                    || *typ == ty(parse_quote!(Expr))
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    let local_struct_names: Vec<_> = node_structs.iter().map(|s| s.ident.clone()).collect();
    let local_struct_types: Vec<syn::Type> = local_struct_names
        .iter()
        .map(|i| parse_quote!(#i))
        .collect();
    let struct_name_regex = local_struct_names
        .iter()
        .rev()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let type_comment_regex = Regex::new(&format!("[lL]ist \\(?of (#{struct_name_regex})")).unwrap();

    struct ReferencesLocalStruct<'a> {
        local_structs: &'a [syn::Type],
        found: bool,
    }

    impl<'ast> Visit<'ast> for ReferencesLocalStruct<'_> {
        fn visit_type(&mut self, node: &'ast syn::Type) {
            if self.local_structs.contains(node) {
                self.found = true
            }
            visit::visit_type(self, node);
        }
    }

    let references_local_struct = |c| {
        let mut visitor = ReferencesLocalStruct {
            local_structs: &local_struct_types,
            found: false,
        };
        visitor.visit_item_const(c);
        visitor.found
    };

    // Keep the safety consts from bindgen
    let safety_consts = file.items.iter().filter_map(|item| match item {
        i @ syn::Item::Const(c) if c.ident == "_" && references_local_struct(c) => Some(i.clone()),
        _ => None,
    });
    out_file.items.extend(safety_consts);

    for mut s in node_structs {
        clean_doc_comments(&mut s.attrs);

        let sname = &s.ident;
        let mut impl_: syn::ItemImpl = parse_quote!(impl #sname {});
        let mut debug_expr: syn::Expr = parse_quote!(f.debug_struct(stringify!(#sname)));

        for field in s.fields.iter_mut() {
            clean_doc_comments(&mut field.attrs);

            let fname = &field.ident;
            let mut fattrs = field.attrs.clone();
            fattrs.push(parse_quote!(#[inline]));
            let debug_kind;

            if field.ty == ty(parse_quote!(NodeTag))
                || field.ty == ty(parse_quote!(Expr))
                || field.ty == ty(parse_quote!(ValUnion))
                || is_flexible_array_ty(&field.ty)
                || matches!(field.ty, syn::Type::Ptr(_))
            {
                field.vis = parse_quote!(pub(crate));
            }

            // Expr* is just Node* with a different name for documentation
            // purposes, but not consistently enough to justify treating it
            // differently
            if field.ty == ty(parse_quote!(*mut Expr)) {
                field.ty = parse_quote!(*mut Node);
            } else if field.ty == ty(parse_quote!(Expr)) {
                field.ty = parse_quote!(NodeTag);
            }

            if let syn::Type::Ptr(ty) = &field.ty
                && local_struct_types.contains(&ty.elem)
            {
                let inner_ty = &ty.elem;
                impl_.items.push(parse_quote! {
                    #(#fattrs)*
                    pub fn #fname(&self) -> Option<&#inner_ty> {
                        // SAFETY: Pointer will always be valid or NULL
                        unsafe { self.#fname.as_ref() }
                    }
                });
                debug_kind = DebugKind::Method;
            } else if field.ty == ty(parse_quote!(*mut List)) {
                let return_ty: syn::Type;
                let mut list_expr: syn::Expr = parse_quote! {
                    // SAFETY: The lifetime is not longer than self
                    unsafe { crate::Node::from_ptr(self.#fname.cast()) }
                };

                let doc_comments = doc_comments(&fattrs)
                    .map(|doc| doc.value())
                    .collect::<Vec<_>>();
                let doc_comments_lower = doc_comments
                    .iter()
                    .flat_map(|doc| doc.lines())
                    .map(|doc| doc.trim().to_lowercase());

                // We assume that parse trees never care about OID, int, or xid
                // lists, so just treat them as plain nodes
                if doc_comments_lower
                    .clone()
                    .any(|doc| doc.starts_with("oid list"))
                {
                    return_ty = parse_quote!(crate::Node<'_>);
                    debug_kind = DebugKind::Skip;
                } else if doc_comments_lower
                    .clone()
                    .any(|doc| doc.starts_with("int list") || doc.starts_with("integer list"))
                {
                    return_ty = parse_quote!(crate::Node<'_>);
                    debug_kind = DebugKind::Skip;
                } else if let Some(captures) = doc_comments
                    .iter()
                    .find_map(|doc| type_comment_regex.captures(&doc))
                {
                    let type_name = &captures[1];
                    let ident = syn::Ident::new(type_name, field.ty.span());
                    return_ty = parse_quote!(&crate::list::CastNodeList<&#ident>);
                    list_expr = parse_quote!(#list_expr.expect_node_list().cast());
                    debug_kind = DebugKind::Method;
                } else {
                    return_ty = parse_quote!(&crate::list::NodeList);
                    list_expr = parse_quote!(#list_expr.expect_node_list());
                    debug_kind = DebugKind::Method;
                }

                impl_.items.push(parse_quote! {
                    #(#fattrs)*
                    pub fn #fname(&self) -> #return_ty {
                        #list_expr
                    }
                });
            } else if field.ty == ty(parse_quote!(*mut Node)) {
                impl_.items.push(parse_quote! {
                    #(#fattrs)*
                    pub fn #fname(&self) -> crate::Node<'_> {
                        // SAFETY: The lifetime is not longer than self
                        unsafe { crate::Node::from_ptr(self.#fname) }
                    }
                });
                debug_kind = DebugKind::Method;
            } else if is_c_string(&field.ty) {
                impl_.items.push(parse_quote! {
                    #(#fattrs)*
                    pub fn #fname(&self) -> Option<&str> {
                        if self.#fname.is_null() {
                            None
                        } else {
                            // SAFETY: PG will always give us a valid string or NULL
                            Some(
                                unsafe { std::ffi::CStr::from_ptr(self.#fname) }
                                    .to_str()
                                    .expect("Parsing is always done in UTF-8"),
                            )
                        }
                    }
                });
                debug_kind = DebugKind::Method;
            } else if field.ty == ty(parse_quote!(ValUnion)) {
                impl_.items.push(parse_quote! {
                    #(#fattrs)*
                    pub fn #fname(&self) -> Option<ConstValue<'_>> {
                        if self.isnull {
                            None
                        } else {
                            Some(ConstValue::from_raw(&self.val))
                        }
                    }
                });
                debug_kind = DebugKind::Method;
            } else if field.ty == parse_quote!(NodeTag) || field.ty == parse_quote!(ParseLoc) {
                debug_kind = DebugKind::Skip;
            } else {
                debug_kind = DebugKind::Field;
            }

            let debug_value: Option<syn::Expr> = match debug_kind {
                DebugKind::Method => Some(parse_quote!(&self.#fname())),
                DebugKind::Field => Some(parse_quote!(&self.#fname)),
                DebugKind::Skip => None,
            };

            if let Some(debug_value) = debug_value {
                debug_expr = parse_quote! {
                    #debug_expr.field(stringify!(#fname), #debug_value)
                };
            }
        }

        out_file.items.push(parse_quote! {
            impl fmt::Debug for #sname {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    #debug_expr.finish_non_exhaustive()
                }
            }
        });

        out_file.items.push(parse_quote! {
            impl<'a> TryFrom<crate::Node<'a>> for &'a #sname {
                type Error = crate::Node<'a>;

                fn try_from(node: crate::Node<'a>) -> Result<Self, Self::Error> {
                    match node {
                        crate::Node::#sname(n) => Ok(n),
                        n => Err(n),
                    }
                }
            }
        });

        out_file.items.push(s.into());
        out_file.items.push(impl_.into());
    }
    std::fs::write(path, prettyplease::unparse(&out_file))?;
    Ok(local_struct_names)
}

fn generate_node_enum(
    node_structs: &[syn::Ident],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut out_file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: Vec::new(),
    };

    let node_tags: Vec<syn::Expr> = node_structs
        .iter()
        .map(|i| {
            let tag_name = syn::Ident::new(&format!("NodeTag_T_{i}"), i.span());
            parse_quote!(raw::#tag_name)
        })
        .collect();

    let enum_variants = node_structs
        .iter()
        .zip(&node_tags)
        .map::<syn::Variant, _>(|(i, tag)| parse_quote!(#i(&'a nodes::#i) = #tag));
    out_file.items.push(parse_quote! {
        #[allow(nonstandard_style)]
        #[repr(u32)]
        #[derive(Debug, Clone, Copy)]
        pub enum Node<'a> {
            /// A null pointer to a node
            None = 0,
            NodeList(&'a crate::list::NodeList) = raw::NodeTag_T_List,
            #(#enum_variants,)*
            /// A pointer to a node that wasn't part of a parse tree, or that
            /// pg_raw_parse doesn't know how to generate code for.
            Invalid(&'a raw::Node),
        }
    });

    out_file.items.push(parse_quote! {
        impl<'a> Node<'a> {
            /// SAFETY: The caller is responsible for ensuring the provided
            /// lifetime does not outlast the memory context this Node was
            /// allocated in
            pub(crate) unsafe fn from_ptr(ptr: *mut raw::Node) -> Self {
                // SAFETY: PG will never return an invalid Node other than NULL
                // and the caller is ensuring a valid lifetime
                unsafe { ptr.as_ref() }.map(|p| {
                    let tag = p.type_;
                    match tag {
                        #(#node_tags => {
                            // SAFETY: We're checking the tag
                            Self::#node_structs(unsafe { &*ptr.cast_const().cast() })
                        })*
                        // SAFETY: We're checking the tag
                        raw::NodeTag_T_List => Self::NodeList(unsafe { &*ptr.cast_const().cast() }),
                        _ => Self::Invalid(p),
                    }
                }).unwrap_or(Self::None)
            }

            pub(crate) fn as_ptr(&self) -> *mut raw::Node {
                match *self {
                    Self::None => std::ptr::null_mut(),
                    Self::Invalid(p) => (&raw const *p).cast_mut(),
                    Self::NodeList(p) => (&raw const *p).cast_mut().cast(),
                    #(Self::#node_structs(p) => (&raw const *p).cast_mut().cast(),)*
                }
            }
        }
    });

    std::fs::write(path, prettyplease::unparse(&out_file))?;
    Ok(())
}

fn is_flexible_array_ty(ty: &syn::Type) -> bool {
    matches!(
        ty,
        syn::Type::Path(
            syn::TypePath { path: syn::Path { segments, .. }, .. },
        ) if segments.first().map(|s| s.ident == "__IncompleteArrayField").unwrap_or(false),
    )
}

fn is_c_string(ty: &syn::Type) -> bool {
    matches!(
        ty,
        syn::Type::Ptr(syn::TypePtr { elem, .. })
            if matches!(&**elem, syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. })
                if segments.last().map(|s| s.ident == "c_char").unwrap_or(false)))
}

fn is_primitive_alias(alias: &syn::ItemType) -> bool {
    alias.ident.to_string().contains("int")
        || matches!(
            &*alias.ty,
            syn::Type::Path(syn::TypePath {
                path: syn::Path { segments, .. },
                ..
            })
                if segments.last().map(|s| {
                    s.ident.to_string().contains("int")
                        || s.ident == "usize"
                        || s.ident == "isize"
                        || s.ident == "Oid"
                        || s.ident == "f32"
                        || s.ident == "f64"
                }).unwrap_or(false)
        )
}

/// For any attributes that are doc comments, clean up characters that will
/// have unintended special meaning in markdown:
///
/// Escape any angle brackets (they aren't intended as HTML tags)
/// Escape any square brackets (they aren't intended as links)
/// Trim any leading whitespace (it isn't intended as a Rust code block)
/// Remove any lines that are entirely - and * (they aren't intended as headers)
fn clean_doc_comments(attrs: &mut [syn::Attribute]) {
    use syn::LitStr;
    for s in doc_comments_mut(attrs) {
        let docstr = s
            .value()
            .replace("<", "\\<")
            .replace(">", "\\>")
            .replace("[", "\\[")
            .replace("]", "\\]")
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.is_empty() || !l.chars().all(|c| c == '-'))
            .collect::<Vec<_>>()
            .join("\n")
            .replace("*/\n/*", "\n");
        *s = LitStr::new(&format!(" {docstr}"), s.span());
    }
}

/// Returns an iterator of pointers to the LitStr value of any doc comment
/// attributes that exist in the list
fn doc_comments_mut<'a>(
    attrs: impl IntoIterator<Item = &'a mut syn::Attribute>,
) -> impl Iterator<Item = &'a mut syn::LitStr> {
    use syn::{Expr, ExprLit, Lit, Meta};

    attrs
        .into_iter()
        .filter_map(|attr| match &mut attr.meta {
            Meta::NameValue(name_value) => Some(name_value),
            _ => None,
        })
        .filter(|nv| nv.path == parse_quote!(doc))
        .filter_map(|nv| match &mut nv.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Some(s),
            _ => None,
        })
}

/// Returns an iterator of pointers to the LitStr value of any doc comment
/// attributes that exist in the list
fn doc_comments<'a>(
    attrs: impl IntoIterator<Item = &'a syn::Attribute>,
) -> impl Iterator<Item = &'a syn::LitStr> {
    use syn::{Expr, ExprLit, Lit, Meta};

    attrs
        .into_iter()
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(name_value) => Some(name_value),
            _ => None,
        })
        .filter(|nv| nv.path == parse_quote!(doc))
        .filter_map(|nv| match &nv.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => Some(s),
            _ => None,
        })
}

/// Type ascription for syn::Type
fn ty(ty: syn::Type) -> syn::Type {
    ty
}

enum DebugKind {
    Method,
    Field,
    Skip,
}
