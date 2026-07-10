use convert_case::{Case, Casing};
use glob::glob;
use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use syn::parse::ParseStream;
use syn::{
    Token, parse_quote,
    spanned::Spanned,
    visit::{self, Visit},
    visit_mut::VisitMut,
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
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .rustified_non_exhaustive_enum("A_Expr_Kind")
        .rustified_non_exhaustive_enum("BoolExprType")
        .rustified_non_exhaustive_enum("SortByDir")
        .rustified_non_exhaustive_enum("SortByNulls")
        .clang_arg(format!("-I{}", build_dir.join("include").display()))
        .clang_arg(format!("-I{}", c_dir.display()))
        .clang_arg(format!(
            "-I{}",
            c_dir.join("src/postgres/include").display()
        ));

    let mut node_bindings = String::new();
    bindgen
        .clone()
        // Exclude Node types that aren't parse nodes and would require
        // additional logic to support
        .blocklist_type("Const")
        .blocklist_type("Expr")
        .blocklist_type("JsonTablePath")
        .blocklist_type("JsonTablePlan")
        .blocklist_type("RelabelType")
        .blocklist_type("Var")
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
    generate_transformer(&node_structs, &out_dir.join("transform_raw.rs")).unwrap();

    let mut makefunc_bindings = String::new();
    bindgen
        .clone()
        .allowlist_file(
            c_dir
                .join("src/postgres/include/nodes/makefuncs.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_file(
            c_dir
                .join("src/postgres/include/nodes/value.h")
                .to_str()
                .unwrap(),
        )
        .blocklist_item("makeColumnDef") // Has more logic than we want
        .blocklist_item("makeTypeName") // We map to the list form, not unqualified
        .generate()
        .unwrap()
        // SAFETY: YOLO
        .write(Box::new(unsafe { makefunc_bindings.as_mut_vec() }))
        .unwrap();
    let make_funcs = generate_make_funcs(&makefunc_bindings, &node_structs, &out_dir).unwrap();

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
        .allowlist_item("StringInfo")
        .allowlist_item("wrapped_raw_deparse")
        .allowlist_item("wrapped_pnstrdup")
        .allowlist_item("list_copy")
        .allowlist_item("lappend")
        .allowlist_item("list_insert_nth")
        .allowlist_item("list_concat")
        .allowlist_item("wrapped_copy_object")
        .allowlist_item("newNode")
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"));
    for struct_name in &node_structs {
        bindgen = bindgen.blocklist_item(struct_name.name.to_string());
    }
    for make_func_name in &make_funcs {
        bindgen = bindgen.allowlist_item(make_func_name);
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

struct NodeStruct {
    attrs: Vec<syn::Attribute>,
    name: syn::Ident,
    fields: Vec<NodeField>,
}

impl NodeStruct {
    fn tag_expr(&self) -> syn::Expr {
        let tag_name = syn::Ident::new(&format!("NodeTag_T_{}", &self.name), self.name.span());
        parse_quote!(raw::#tag_name)
    }

    fn mut_ref_name(&self) -> syn::Ident {
        syn::Ident::new(&format!("{}Mut", &self.name), self.name.span())
    }

    fn transform_function_name(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("transform_{}", self.name.to_string().to_case(Case::Snake)),
            self.name.span(),
        )
    }
}

struct NodeField {
    attrs: Vec<syn::Attribute>,
    name: syn::Ident,
    ty: NodeFieldType,
}

impl NodeField {
    fn vis(&self) -> syn::Visibility {
        match self.ty {
            NodeFieldType::Primitive(_) => parse_quote!(pub),
            NodeFieldType::CString => parse_quote!(pub(crate)),
            NodeFieldType::Private(_) if self.name == "type_" => parse_quote!(pub(crate)),
            _ => syn::Visibility::Inherited,
        }
    }

    fn accessor_method(&self) -> Option<syn::ImplItem> {
        use NodeFieldType::*;

        let fattrs = &self.attrs;
        let fname = &self.name;
        let ret = self.ty(&parse_quote!('_));
        match &self.ty {
            Private(_) | Primitive(_) => None,

            Node | CastNode(_) | List | CastList(_) => Some(parse_quote! {
                #(#fattrs)*
                #[inline]
                pub fn #fname(&self) -> #ret {
                    // SAFETY: The lifetime is not longer than self
                    unsafe { crate::FromNodePtr::from_raw(self.#fname.cast()) }
                }
            }),

            CString => Some(parse_quote! {
                #(#fattrs)*
                #[inline]
                pub fn #fname(&self) -> #ret {
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
            }),

            ConstVal => Some(parse_quote! {
                #(#fattrs)*
                #[inline]
                pub fn #fname(&self) -> #ret {
                    if self.isnull {
                        None
                    } else {
                        Some(ConstValue::from_raw(&self.val))
                    }
                }
            }),
        }
    }

    fn mut_accessor(&self, mem: &syn::Lifetime) -> Option<syn::ItemFn> {
        use NodeFieldType::*;

        let fname = &self.name;
        let func_name = syn::Ident::new(&format!("{fname}_mut"), fname.span());
        let inner = &self.ty(&mem);
        match &self.ty {
            // No reason to provide for these, user can just `&mut node.field`
            Private(_) | Primitive(_) => None,

            Node | CastNode(_) | List | CastList(_) => Some(parse_quote! {
                #[inline]
                pub fn #func_name(&mut self) -> <#inner as FromNodeMut<#mem>>::MutRef<'_> {
                    // SAFETY: The pointer will always be valid or NULL.
                    // The lifetime won't outlive self
                    unsafe {
                        <#inner as FromNodeMut>::from_ptr_mut(&mut self.mut_ref.#fname, self.id)
                    }
                }
            }),

            // We can't provide mutable strings, and there's no reason to.
            CString => None,

            // Mutable access to this would basically be mutating either a
            // primitive or a string. We don't provide mut accessors for either
            // of those, so we don't provide them for this
            ConstVal => None,
        }
    }

    fn setter_method(&self, self_expr: syn::Expr, mem: &syn::Lifetime) -> Option<syn::ImplItem> {
        let fname = &self.name;
        let func_name = syn::Ident::new(&format!("set_{}", fname), fname.span());
        let ty = self.setter_ty(&mem)?;
        let set_expr = self.setter_expr();
        Some(parse_quote! {
            #[inline]
            pub fn #func_name(&mut self, #fname: #ty) {
                #self_expr.#fname = #set_expr;
            }
        })
    }

    fn debug_expr(&self, debug_expr: syn::Expr) -> syn::Expr {
        use NodeFieldType::*;

        let fname = &self.name;
        let value_expr: syn::Expr = match &self.ty {
            Primitive(_) => parse_quote!(&self.#fname),
            Node | CastNode(_) | List | CastList(_) | CString | ConstVal => {
                parse_quote!(&self.#fname())
            }
            Private(_) => return debug_expr,
        };

        parse_quote!(#debug_expr.field(stringify!(#fname), #value_expr))
    }

    fn raw_ty(&self) -> syn::Type {
        self.ty.raw_ty()
    }

    fn ty(&self, lifetime: &syn::Lifetime) -> syn::Type {
        self.ty.ty(lifetime)
    }

    fn constructor_ty(&self, lifetime: &syn::Lifetime) -> Option<syn::Type> {
        self.ty.constructor_ty(lifetime)
    }

    fn setter_ty(&self, lifetime: &syn::Lifetime) -> Option<syn::Type> {
        use NodeFieldType::*;

        match self.ty {
            CString => Some(parse_quote!(Option<PgStr<#lifetime>>)),
            _ => self.constructor_ty(lifetime),
        }
    }

    fn as_raw_expr(&self) -> syn::Expr {
        use NodeFieldType::*;

        let fname = &self.name;
        match self.ty {
            Primitive(_) | ConstVal => parse_quote!(#fname),
            Private(_) => parse_quote!(Default::default()),
            Node | CastNode(_) | List | CastList(_) => parse_quote!(#fname.into_ptr().cast()),
            CString => parse_quote! {
                #fname
                    .map(|s| self.copy_string(s).into_ptr())
                    .unwrap_or(ptr::null_mut())
            },
        }
    }

    fn setter_expr(&self) -> syn::Expr {
        use NodeFieldType::*;

        let fname = &self.name;
        match self.ty {
            CString => parse_quote! {
                #fname
                    .map(|f| f.into_ptr())
                    .unwrap_or(std::ptr::null_mut())
            },
            _ => self.as_raw_expr(),
        }
    }

    fn transform_stmt(
        &self,
        lt: &syn::Lifetime,
        transformer: &syn::Expr,
        node: &syn::Expr,
    ) -> Option<syn::Stmt> {
        use NodeFieldType::*;

        let mut_accessor = self.mut_accessor(lt)?.sig.ident;
        match &self.ty {
            Node => {
                Some(parse_quote!(#transformer.transform_node(Assignable(#node.#mut_accessor()));))
            }
            CastNode(inner) => {
                let trans_fname =
                    syn::Ident::new(&format!("transform_{}", snake_case(inner)), inner.span());
                Some(parse_quote! {
                    if let Some(node) = #node.#mut_accessor() {
                        #transformer.#trans_fname(node);
                    }
                })
            }
            List => Some(parse_quote!(#transformer.transform_list(#node.#mut_accessor());)),
            CastList(inner) => {
                let trans_fname = syn::Ident::new(
                    &format!("transform_{}_list", snake_case(inner)),
                    inner.span(),
                );
                Some(parse_quote!(#transformer.#trans_fname(#node.#mut_accessor());))
            }

            Private(_) | Primitive(_) | ConstVal | CString => None,
        }
    }
}

enum NodeFieldType {
    Private(syn::Type),
    Node,
    CastNode(syn::Ident),
    List,
    CastList(syn::Ident),
    CString,
    ConstVal,
    Primitive(syn::Type),
}

impl NodeFieldType {
    fn raw_ty(&self) -> syn::Type {
        match self {
            Self::Private(t) | Self::Primitive(t) => t.clone(),
            Self::Node | Self::CastNode(_) | Self::List | Self::CastList(_) => {
                parse_quote!(*mut Node)
            }
            Self::CString => parse_quote!(*mut std::ffi::c_char),
            Self::ConstVal => parse_quote!(ValUnion),
        }
    }

    fn ty(&self, lifetime: &syn::Lifetime) -> syn::Type {
        match self {
            Self::Private(t) | Self::Primitive(t) => t.clone(),
            Self::Node => parse_quote!(crate::Node<#lifetime>),
            Self::CastNode(t) => parse_quote!(Option<&#lifetime crate::nodes::#t>),
            Self::List => parse_quote!(&#lifetime crate::list::NodeList),
            Self::CastList(t) => parse_quote!(&#lifetime crate::list::CastNodeList<#t>),
            Self::CString => parse_quote!(Option<&#lifetime str>),
            Self::ConstVal => parse_quote!(Option<crate::const_val::ConstValue<#lifetime>>),
        }
    }

    fn constructor_ty(&self, mem: &syn::Lifetime) -> Option<syn::Type> {
        let inner = self.ty(&parse_quote!('_));
        match self {
            Self::Private(_) => None,
            Self::Primitive(_) => Some(inner),
            Self::ConstVal => Some(parse_quote!(crate::raw::ValUnion)),
            Self::Node | Self::CastNode(_) | Self::List | Self::CastList(_) => {
                Some(parse_quote!(Unique<#mem, #inner>))
            }
            // Strings get copied in constructors so we can ignore the input LT
            Self::CString => Some(parse_quote!(Option<&str>)),
        }
    }
}

/// Generates the structs for each node and writes them to the given path.
/// Returns a list of the struct names generated
fn generate_node_structs(
    bindings: &str,
    path: &Path,
) -> Result<Vec<NodeStruct>, Box<dyn std::error::Error>> {
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

    let raw_node_structs = file
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
                }) if *typ == parse_quote!(NodeTag)
                    || *typ == parse_quote!(Expr)
            )
        });
    let local_struct_names = raw_node_structs
        .clone()
        .map(|s| &s.ident)
        .collect::<Vec<_>>();
    let struct_name_regex = local_struct_names
        .iter()
        .rev()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let type_comment_regex = Regex::new(&format!("[lL]ist \\(?of (#{struct_name_regex})")).unwrap();

    let node_structs = raw_node_structs
        .map(|s| build_node_struct(s, &type_comment_regex))
        .collect::<Vec<_>>();

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

    let local_struct_types: Vec<syn::Type> = local_struct_names
        .iter()
        .map(|i| parse_quote!(#i))
        .collect();
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

    struct FixCasing;
    impl VisitMut for FixCasing {
        fn visit_macro_mut(&mut self, i: &mut syn::Macro) {
            if i.path == parse_quote!(::std::mem::offset_of) {
                let (ty, field) = i
                    .parse_body_with(|input: ParseStream| {
                        let ty = input.parse::<syn::Type>()?;
                        input.parse::<Token![,]>()?;
                        let field = input.parse::<syn::Ident>()?;
                        Ok((ty, field))
                    })
                    .unwrap();
                let field = snake_case(&field);
                i.tokens = parse_quote!(#ty, #field);
            }
        }
    }
    let safety_consts = safety_consts.map(|mut c| {
        FixCasing.visit_item_mut(&mut c);
        c
    });
    out_file.items.extend(safety_consts);

    for s in &node_structs {
        let sattrs = &s.attrs;
        let sname = &s.name;
        let smut = s.mut_ref_name();

        for f in &s.fields {
            if let NodeFieldType::CastNode(t) = &f.ty
                && !local_struct_names.contains(&t)
            {
                panic!(
                    "{sname}.{} is a pointer to {t:?}, which is not a Node. It needs special handling",
                    f.name
                )
            }
        }

        let fattrs = s.fields.iter().map(|f| &f.attrs);
        let fvis = s.fields.iter().map(|f| f.vis());
        let fnames = s.fields.iter().map(|f| &f.name);
        let ftys = s.fields.iter().map(|f| f.raw_ty());
        out_file.items.push(parse_quote! {
            #(#sattrs)*
            pub struct #sname {
                #(#(#fattrs)* #fvis #fnames: #ftys,)*
            }
        });

        let accessors = s.fields.iter().filter_map(|f| f.accessor_method());
        out_file.items.push(parse_quote! {
            impl #sname {
                #(#accessors)*
            }
        });

        let debug_expr = s.fields.iter().fold(
            parse_quote!(f.debug_struct(stringify!(#sname))),
            |expr, field| field.debug_expr(expr),
        );
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

        out_file.items.push(parse_quote! {
            impl<'a> From<&'a #sname> for crate::Node<'a> {
                fn from(node: &'a #sname) -> Self {
                    Self::#sname(node)
                }
            }
        });

        out_file.items.push(parse_quote! {
            impl<'a> From<Option<&'a #sname>> for crate::Node<'a> {
                fn from(node: Option<&'a #sname>) -> Self {
                    match node {
                        Some(node) => Self::from(node),
                        None => Self::None,
                    }
                }
            }
        });

        let tag = s.tag_expr();
        out_file.items.push(parse_quote! {
            impl crate::ConstructableNode for #sname {
                const TAG: NodeTag = #tag;
            }
        });

        out_file.items.push(parse_quote! {
            impl<'a> crate::FromNodePtr for &'a #sname {
                unsafe fn from_ptr(tag: NodeTag, ptr: Option<NonNull<Node>>) -> Self {
                    if tag == #sname::TAG {
                        let p = ptr.expect("Unexpected NULL ptr")
                            .cast();
                        // SAFETY: We've checked the tag
                        unsafe { p.as_ref() }
                    } else {
                        panic!(concat!("Expected a ", stringify!(#sname), "got tag {}"), tag)
                    }
                }
            }
        });

        out_file.items.push(parse_quote! {
            impl<'mem> crate::FromNodeMut<'mem> for &#sname {
                type MutRef<'mutref> = #smut<'mem, 'mutref>;

                unsafe fn from_ptr_mut<'mutref>(
                    ptr: &'mutref mut *mut Node,
                    id: Id<'mem>,
                ) -> Self::MutRef<'mutref> {
                    if ptr.is_null() {
                        panic!("from_ptr_mut called on a NULL pointer")
                    }

                    // SAFETY: Caller is responsible for making this safe
                    let mut_ref = unsafe {
                        std::ptr::from_mut(ptr)
                            .cast::<&mut #sname>()
                            .as_mut()
                            .unwrap()
                    };
                    #smut { id, mut_ref }
                }
            }
        });

        out_file.items.push(parse_quote! {
            impl crate::AsNodeRef for #sname {
                type AsRef<'b> = &'b #sname;
                type List = crate::list::CastNodeList<#sname>;
            }
        });

        out_file.items.push(parse_quote! {
            // SAFETY: Self is a type of node
            unsafe impl<'a> crate::AsNodePtr for &'a #sname {
                fn as_ptr(self) -> *mut Node {
                    std::ptr::from_ref(self).cast_mut().cast()
                }
            }
        });

        out_file.items.push(parse_quote! {
            // SAFETY: No reason we can't share nodes across threads
            unsafe impl Send for #sname {}
        });

        out_file.items.push(parse_quote! {
            // SAFETY: No reason we can't share nodes across threads
            unsafe impl Sync for #sname {}
        });

        out_file.items.push(parse_quote! {
            #[allow(non_camel_case_types)]
            pub struct #smut<'mem, 'mutref> {
                pub(crate) id: Id<'mem>,
                mut_ref: &'mutref mut &'mutref mut #sname,
            }
        });

        let setters = s
            .fields
            .iter()
            .filter_map(|f| f.setter_method(parse_quote!(self.mut_ref), &parse_quote!('mem)));
        let mut_accessors = s
            .fields
            .iter()
            .filter_map(|f| f.mut_accessor(&parse_quote!('mem)));
        out_file.items.push(parse_quote! {
            impl<'mem, 'mutref> #smut<'mem, 'mutref> {
                #(#setters)*
                #(#mut_accessors)*

                pub fn replace(self, node: Unique<'mem, &#sname>) {
                    let ptr = node.into_ptr().cast::<#sname>();
                    // SAFETY: ptr is always a valid pointer or NULL
                    let new = unsafe { ptr.as_mut() }
                        .expect(concat!("Cannot replace a ", stringify!(#sname), " with NULL"));
                    *self.mut_ref = new;
                }

                pub(crate) fn into_assignment(self) -> *mut *mut raw::Node {
                    std::ptr::from_mut(self.mut_ref).cast()
                }
            }
        });

        out_file.items.push(parse_quote! {
            impl<'mem, 'mutref> std::ops::Deref for #smut<'mem, 'mutref> {
                type Target = #sname;

                fn deref(&self) -> &Self::Target {
                    &*self.mut_ref
                }
            }
        });

        out_file.items.push(parse_quote! {
            impl<'mem, 'mutref> From<#smut<'mem, 'mutref>> for crate::NodeMut<'mem, 'mutref> {
                fn from(node: #smut<'mem, 'mutref>) -> Self {
                    Self::#sname(node)
                }
            }
        });
    }

    std::fs::write(path, prettyplease::unparse(&out_file))?;
    Ok(node_structs)
}

fn generate_node_enum(
    node_structs: &[NodeStruct],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut out_file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: Vec::new(),
    };

    let node_names = node_structs.iter().map(|s| &s.name).collect::<Vec<_>>();
    let enum_variants = node_names
        .iter()
        .map::<syn::Variant, _>(|i| parse_quote!(#i(&'a nodes::#i) = nodes::#i::TAG));
    out_file.items.push(parse_quote! {
        #[allow(non_camel_case_types)]
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
        impl<'a> FromNodePtr for Node<'a> {
            /// SAFETY: The caller is responsible for ensuring the provided
            /// lifetime does not outlast the memory context this Node was
            /// allocated in
            unsafe fn from_ptr(tag: raw::NodeTag, ptr: Option<NonNull<raw::Node>>) -> Self {
                // SAFETY: PG will never return an invalid Node other than NULL
                // and the caller is ensuring a valid lifetime
                match (tag, ptr) {
                    (_, None) => Self::None,
                    #((nodes::#node_names::TAG, Some(ptr)) => {
                        debug_assert!(ptr.cast::<nodes::#node_names>().is_aligned());
                        // SAFETY: We're checking the tag
                        Self::#node_names(unsafe { ptr.cast().as_ref() })
                    })*
                    // SAFETY: We're checking the tag
                    (raw::NodeTag_T_List, Some(ptr)) => {
                        debug_assert!(ptr.cast::<list::NodeList>().is_aligned());
                        Self::NodeList(unsafe { ptr.cast().as_ref() })
                    }
                    (_, Some(p)) => Self::Invalid(unsafe { p.as_ref() }),
                }
            }
        }
    });

    out_file.items.push(parse_quote! {
        impl<'a> AsNodeRef for Node<'a> {
            type AsRef<'b> = Node<'b>;
            type List = crate::list::NodeList;
        }
    });

    out_file.items.push(parse_quote! {
        // SAFETY: We are returning the inner pointer from as_ptr
        unsafe impl<'a> AsNodePtr for Node<'a> {
            fn as_ptr(self) -> *mut raw::Node {
                match self {
                    Self::None => std::ptr::null_mut(),
                    Self::Invalid(p) => (&raw const *p).cast_mut(),
                    Self::NodeList(p) => (&raw const *p).cast_mut().cast(),
                    #(Self::#node_names(p) => (&raw const *p).cast_mut().cast(),)*
                }
            }
        }
    });

    out_file.items.push(parse_quote! {
        impl<'mem> FromNodeMut<'mem> for Node<'_> {
            type MutRef<'mutref> = NodeMut<'mem, 'mutref>;

            unsafe fn from_ptr_mut<'mutref>(
                ptr: &'mutref mut *mut raw::Node,
                id: Id<'mem>,
            ) -> Self::MutRef<'mutref> {
                // SAFETY: Pointer is not null. Caller is responsible for making
                // this safe.
                unsafe {
                    match ptr.as_ref().map(|p| p.type_) {
                        None => NodeMut::None(ptr, id),
                        #(Some(nodes::#node_names::TAG) => {
                            NodeMut::#node_names(<&'mem nodes::#node_names>::from_ptr_mut(ptr, id))
                        })*
                        Some(raw::NodeTag_T_List) => NodeMut::NodeList(<&'mem crate::list::NodeList>::from_ptr_mut(ptr, id)),
                        Some(_) => NodeMut::Invalid(ptr, id),
                    }
                }
            }
        }
    });

    let enum_variants = node_names.iter().map::<syn::Variant, _>(
        |i| parse_quote!(#i(<&'mem nodes::#i as FromNodeMut<'mem>>::MutRef<'mutref>) = nodes::#i::TAG),
    );
    out_file.items.push(parse_quote! {
        #[allow(non_camel_case_types)]
        #[repr(u32)]
        pub enum NodeMut<'mem, 'mutref> {
            /// A NULL pointer to a node
            None(&'mutref mut *mut raw::Node, Id<'mem>) = 0,
            NodeList(<&'mem crate::list::NodeList as FromNodeMut<'mem>>::MutRef<'mutref>) = raw::NodeTag_T_List,
            #(#enum_variants,)*
            /// A pointer to a node that wasn't part of a parse tree, or that
            /// pg_raw_parse doesn't know how to generate code for.
            Invalid(&'mutref mut *mut raw::Node, Id<'mem>),
        }
    });

    out_file.items.push(parse_quote! {
        impl<'mem, 'mutref> NodeMut<'mem, 'mutref> {
            /// Returns the lifetime brand for the memory context this points
            /// to
            pub(crate) fn id(&self) -> Id<'mem> {
                match self {
                    Self::None(_, id) => *id,
                    Self::NodeList(list) => list.id,
                    #(Self::#node_names(n) => n.id,)*
                    Self::Invalid(_, id) => *id,
                }
            }

            /// Get the raw pointer representation of this node
            pub(crate) fn into_ptr(self) -> *mut raw::Node {
                match self {
                    Self::None(..) => std::ptr::null_mut(),
                    Self::NodeList(mut list) => list.take_ptr().cast(),
                    #(Self::#node_names(p) => p.as_ptr(),)*
                    Self::Invalid(p, _) => *p,
                }
            }

            /// Assign a node in-place
            ///
            /// # Safety
            ///
            /// This NodeMut must not have been constructed from a node that
            /// is expected to be of a specific type
            pub(crate) unsafe fn replace(self, node: make::Unique<'mem, Node<'_>>) {
                let ref_mut = match self {
                    Self::None(ref_mut, _) => ptr::from_mut(ref_mut).cast(),
                    Self::NodeList(list) => list.into_assignment(),
                    #(Self::#node_names(p) => p.into_assignment(),)*
                    Self::Invalid(ref_mut, _) => ptr::from_mut(ref_mut),
                };
                // SAFETY: Caller is responsible for making this safe
                unsafe { *ref_mut = node.into_ptr() };
            }
        }
    });

    std::fs::write(path, prettyplease::unparse(&out_file))?;
    Ok(())
}

/// Returns the function names needed in `raw`
fn generate_make_funcs(
    bindings: &str,
    node_structs: &[NodeStruct],
    out_dir: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = syn::parse_file(bindings)?;

    let makefuncs = file
        .items
        .into_iter()
        .flat_map(|i| match i {
            syn::Item::ForeignMod(f) => f.items,
            _ => Vec::new(),
        })
        .filter_map(|i| match i {
            syn::ForeignItem::Fn(f) => Some(f),
            _ => None,
        })
        .filter_map(|f| {
            let fname = f.sig.ident.to_string();
            if fname.starts_with("make")
                && let Some(s) = node_structs.iter().find(|s| s.name == &fname[4..])
            {
                Some((s, f))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mem = parse_quote!('mem);
    let items = makefuncs.iter().map(|(node, makefunc)| -> syn::ImplItem {
        let node_name = &node.name;
        let raw_func_name = &makefunc.sig.ident;
        let func_name = snake_case(raw_func_name);

        let arg_fields = makefunc
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(pat_type) => Some(pat_type),
                _ => None,
            })
            .filter_map(|arg| {
                /// The arity of the constructor functions sometimes varies
                /// wildly from the number of fields present on the struct.
                /// Because of that, we get the field an argument maps to by
                /// name instead of index. But in a handful of cases, those
                /// names don't match up, so we have a hard coded list of
                /// corrections
                static MISMATCHED_FIELD_NAMES: &[((&str, &str), &str)] = &[
                    (("BitString", "str_"), "bsval"),
                    (("Boolean", "val"), "boolval"),
                    (("DefElem", "name"), "defname"),
                    (("Float", "numericStr"), "fval"),
                    (("FuncCall", "name"), "funcname"),
                    (("FuncExpr", "fformat"), "funcformat"),
                    (("FuncExpr", "rettype"), "funcresulttype"),
                    (("Integer", "i"), "ival"),
                    (("JsonTablePath", "pathname"), "name"),
                    (("JsonTablePath", "pathvalue"), "value"),
                    (("JsonTablePathSpec", "string_location"), "location"),
                    (("String", "str_"), "sval"),
                ];

                let syn::Pat::Ident(syn::PatIdent { ident: arg, .. }) = &*arg.pat else {
                    return None;
                };
                let arg = MISMATCHED_FIELD_NAMES
                    .iter()
                    .find_map(|((sname, argname), fname)| {
                        (node_name == sname && *arg == argname).then(|| (*fname).to_owned())
                    })
                    .unwrap_or_else(|| arg.to_string());

                node.fields.iter().find(|f| f.name == arg)
            })
            .collect::<Vec<_>>();

        let fargs = arg_fields.iter().filter_map(|field| {
            let fname = &field.name;
            let fty = field.constructor_ty(&mem)?;
            Some::<syn::FnArg>(parse_quote!(#fname: #fty))
        });
        let farg_exprs = arg_fields.iter().map(|field| field.as_raw_expr());

        parse_quote! {
            pub fn #func_name(self, #(#fargs,)*) -> Unique<#mem, &#mem crate::nodes::#node_name> {
                // SAFETY: The given closure never panics. The function raw
                // functions we call are only allocating and assigning fields.
                // They have no error conditions, so we can never longjmp
                // over Rust frames. We have explicitly taken a mut reference
                // to MemoryContext to ensure the lifetime is invariant
                let ptr = unsafe { self.mem.within(|| {
                    raw::#raw_func_name(#(#farg_exprs),*)
                }) };
                Unique(ptr.cast(), self.id, PhantomData)
            }
        }
    });

    std::fs::write(
        out_dir.join("make_funcs_raw.rs"),
        prettyplease::unparse(&parse_quote! {
            impl<#mem> MemoryToken<#mem> {
                #(#items)*
            }
        }),
    )?;
    Ok(makefuncs
        .into_iter()
        .map(|(_, f)| f.sig.ident.to_string())
        .collect())
}

fn generate_transformer(
    node_structs: &[NodeStruct],
    out_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut out_file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: Vec::new(),
    };

    let mut transform_trait: syn::ItemTrait = parse_quote! {
        pub trait Transform<'mem> {
            fn transform_node<'mutref>(&mut self, node: Assignable<'mem, 'mutref>) {
                transform_node(node, self);
            }

            fn transform_list<'mutref>(&mut self, list: NodeListMut<'mem, 'mutref, NodeList>) {
                transform_list(list, self);
            }
        }
    };

    let node_names = node_structs.iter().map(|s| &s.name);
    let trans_fnames = node_structs.iter().map(|s| s.transform_function_name());
    out_file.items.push(parse_quote! {
        pub fn transform_node<'mem, 'mutref, T>(
            node: Assignable<'mem, 'mutref>,
            transformer: &mut T,
        )
        where
            T: Transform<'mem> + ?Sized,
        {
            match node.0 {
                NodeMut::None(..) | NodeMut::Invalid(..) => {},
                NodeMut::NodeList(list) => transformer.transform_list(list),
                #(NodeMut::#node_names(node) => transformer.#trans_fnames(node),)*
            }
        }
    });

    out_file.items.push(parse_quote! {
        pub fn transform_list<'mem, 'mutref, T>(
            list: NodeListMut<'mem, 'mutref, NodeList>,
            transformer: &mut T,
        )
        where
            T: Transform<'mem> + ?Sized,
        {
            for node in list {
                transformer.transform_node(Assignable(node));
            }
        }
    });

    for s in node_structs {
        let trans_fname = s.transform_function_name();
        let sname = &s.name;
        let smut = s.mut_ref_name();

        // i.e. Transform::transform_string
        transform_trait.items.push(parse_quote! {
            fn #trans_fname<'mutref>(&mut self, node: nodes::#smut<'mem, 'mutref>) {
                #trans_fname(node, self)
            }
        });

        let list_trans_fname = syn::Ident::new(&format!("{trans_fname}_list"), trans_fname.span());
        // i.e. Transform::transform_string_list
        transform_trait.items.push(parse_quote! {
            fn #list_trans_fname<'mutref>(
                &mut self,
                node: NodeListMut<'mem, 'mutref, CastNodeList<nodes::#sname>>,
            ) {
                #list_trans_fname(node, self)
            }
        });

        // i.e. transform::transform_string_list
        out_file.items.push(parse_quote! {
            fn #list_trans_fname<'mem, 'mutref, T>(
                list: NodeListMut<'mem, 'mutref, CastNodeList<nodes::#sname>>,
                transformer: &mut T,
            )
            where
                T: Transform<'mem> + ?Sized,
            {
                for node in list {
                    transformer.#trans_fname(node);
                }
            }
        });

        let transform_stmts = s.fields.iter().filter_map(|f| {
            f.transform_stmt(
                &parse_quote!('mem),
                &parse_quote!(transformer),
                &parse_quote!(node),
            )
        });
        // i.e. transform::transform_string
        out_file.items.push(parse_quote! {
            pub fn #trans_fname<'mem, 'mutref, T>(
                // These arguments will be unused for leaf nodes
                #[allow(unused)]
                mut node: nodes::#smut<'mem, 'mutref>,
                #[allow(unused)]
                transformer: &mut T,
            )
            where
                T: Transform<'mem> + ?Sized,
            {
                #(#transform_stmts)*
            }
        });
    }

    out_file.items.push(transform_trait.into());

    std::fs::write(out_path, prettyplease::unparse(&out_file))?;
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
fn clean_doc_comments(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
    attrs
        .iter()
        .map(|a| {
            if let Some(s) = doc_comment(a) {
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
                let s = syn::LitStr::new(&format!(" {docstr}"), s.span());
                parse_quote!(#[doc = #s])
            } else {
                a.clone()
            }
        })
        .collect()
}

/// Returns an iterator of pointers to the LitStr value of any doc comment
/// attributes that exist in the list
fn doc_comments<'a>(
    attrs: impl IntoIterator<Item = &'a syn::Attribute>,
) -> impl Iterator<Item = &'a syn::LitStr> {
    attrs.into_iter().filter_map(doc_comment)
}

fn doc_comment(attr: &syn::Attribute) -> Option<&syn::LitStr> {
    use syn::{Expr, ExprLit, Lit, Meta};
    if let Meta::NameValue(nv) = &attr.meta
        && let Some(path) = nv.path.get_ident()
        && path == "doc"
        && let Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) = &nv.value
    {
        Some(s)
    } else {
        None
    }
}

fn build_node_struct(s: &syn::ItemStruct, type_comment_regex: &Regex) -> NodeStruct {
    let attrs = clean_doc_comments(&s.attrs);
    let name = s.ident.clone();
    let fields = s
        .fields
        .iter()
        .map(|f| {
            let attrs = clean_doc_comments(&f.attrs);
            let name = snake_case(f.ident.as_ref().expect("C doesn't have unnamed fields"));
            let ty = determine_field_ty(f, type_comment_regex);
            NodeField { attrs, name, ty }
        })
        .collect();
    NodeStruct {
        attrs,
        name,
        fields,
    }
}

fn determine_field_ty(field: &syn::Field, comment_regex: &Regex) -> NodeFieldType {
    if field.ty == parse_quote!(*mut Node) || field.ty == parse_quote!(*mut Expr) {
        NodeFieldType::Node
    } else if field.ty == parse_quote!(Expr) || field.ty == parse_quote!(NodeTag) {
        NodeFieldType::Private(parse_quote!(NodeTag))
    } else if field.ty == parse_quote!(*mut List) {
        determine_list_field_ty(field, comment_regex)
    } else if is_c_string(&field.ty) {
        NodeFieldType::CString
    } else if field.ty == parse_quote!(ValUnion) {
        NodeFieldType::ConstVal
    } else if let syn::Type::Ptr(ty) = &field.ty
        && let syn::Type::Path(p) = &*ty.elem
        && let Some(i) = p.path.get_ident()
    {
        // At this point any pointers we haven't yet matched should just be
        // specific types of nodes
        NodeFieldType::CastNode(i.clone())
    } else if field.ty == parse_quote!(ParseLoc) || is_flexible_array_ty(&field.ty) {
        NodeFieldType::Private(field.ty.clone())
    } else {
        NodeFieldType::Primitive(field.ty.clone())
    }
}

fn determine_list_field_ty(field: &syn::Field, comment_regex: &Regex) -> NodeFieldType {
    let doc_comments = doc_comments(&field.attrs)
        .map(|doc| doc.value())
        .collect::<Vec<_>>();
    let mut doc_comments_lower = doc_comments
        .iter()
        .flat_map(|doc| doc.lines())
        .map(|doc| doc.trim().to_lowercase());

    if doc_comments_lower.any(|doc| {
        doc.starts_with("oid list")
            || doc.starts_with("int list")
            || doc.starts_with("integer list")
    }) {
        // We assume that parse trees never care about OID, int, or xid
        // lists, so just treat them as plain nodes
        NodeFieldType::Private(field.ty.clone())
    } else if let Some(captures) = doc_comments
        .iter()
        .find_map(|doc| comment_regex.captures(doc))
    {
        // We found a comment containing "list of NodeType". Assume the
        // comment isn't lying, cast the list to NodeType
        let type_name = &captures[1];
        let ident = syn::Ident::new(type_name, field.ty.span());
        NodeFieldType::CastList(ident)
    } else {
        // Polymorphic list or list without adequate documentation
        NodeFieldType::List
    }
}

fn snake_case(input: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&input.to_string().to_case(Case::Snake), input.span())
}
