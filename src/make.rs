use crate::list::{CastNodeList, NodeList};
use crate::mem::MemoryContext;
use crate::raw::{self, *};
use crate::{AsNodePtr, ConstValue, ConstructableNode, FromNodeMut, Node, Owned, nodes};
use generativity::Id;
use std::any::type_name;
use std::ffi::{c_char, c_int};
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

include!(concat!(env!("OUT_DIR"), "/make_funcs_raw.rs"));

/// A token used to make nodes allocated onto a specific memory context.
/// This type ensures that nodes from one memory context cannot be used as
/// fields of nodes from another.
///
/// FIXME(sage): These tests don't assert that the failures are lifetime
/// related
///
/// ```compile_fail
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     let node = mem1.make_String(Some("hi"));
///     owned(|mem2| mem2.make_List(&[node])); // Fails, node is on mem1
///     mem1.make_String(Some("lol"))
/// });
/// ```
///
/// ```
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     let node = mem1.make_String(Some("hi"));
///     mem1.make_List(&[node]) // Is fine, both nodes are on mem1
/// });
/// ```
#[derive(Clone, Copy)]
pub struct MemoryToken<'a> {
    mem: &'a MemoryContext,
    id: Id<'a>,
}

impl<'a> MemoryToken<'a> {
    #[allow(non_snake_case)]
    pub fn make_A_Const(self, val: ConstValue) -> Unique<'a, &'a nodes::A_Const> {
        use ConstValue::*;

        let mut node = self.make_node::<nodes::A_Const>();
        let node_ref = node.as_mut().into_inner();
        node_ref.isnull = false;
        // SAFETY: We're never casting to anything other than node without
        // checking the tag
        unsafe {
            let v = &mut node_ref.val;
            v.node.type_ = val.tag();
            match val {
                Integer(i) => v.ival.ival = i,
                Boolean(b) => v.boolval.boolval = b,
                // These are all the same repr, so it doesn't matter which
                // variant we assign the string pointer to as long as we set
                // the tag correctly.
                Float(s) | String(s) | BitString(s) => v.sval.sval = self.copy_string(s),
                Unrecognized(_) => panic!("Cannot create A_Const with unrecognized value"),
            }
        }
        node
    }

    #[allow(non_snake_case)]
    pub fn make_ColumnRef(
        self,
        fields: Unique<'a, &'a NodeList>,
    ) -> Unique<'a, &'a nodes::ColumnRef> {
        let mut node = self.make_node::<nodes::ColumnRef>();
        node.as_mut().set_fields(fields);
        node
    }

    #[allow(non_snake_case)]
    pub fn make_CommonTableExpr(
        self,
        ctename: &str,
        aliascolnames: Unique<'a, &'a NodeList>,
        ctequery: Unique<'a, Node<'a>>,
    ) -> Unique<'a, &'a nodes::CommonTableExpr> {
        let mut cte = self.make_node::<nodes::CommonTableExpr>();
        cte.as_mut().into_inner().ctename = self.copy_string(ctename);
        cte.as_mut().set_aliascolnames(aliascolnames);
        cte.as_mut().set_ctequery(ctequery);
        cte
    }

    #[allow(non_snake_case)]
    pub fn make_List<T: AsNodePtr>(self, elems: &[Unique<'a, T>]) -> Unique<'a, &'a T::List> {
        if elems.is_empty() {
            Unique(ptr::null_mut(), self.id, PhantomData)
        } else {
            let list_to_copy = raw::List {
                type_: raw::NodeTag_T_List,
                length: elems.len() as _,
                max_length: elems.len() as _,
                elements: elems.as_ptr().cast_mut().cast(),
                initial_elements: raw::__IncompleteArrayField::new(),
            };
            // SAFETY: The given closure never panics, we're passing valid pointers
            let ptr = unsafe { self.mem.within(|| raw::list_copy(&raw const list_to_copy)) };
            // SAFETY: The returned pointer is always a palloc'd list pointer
            Unique(ptr.cast(), self.id, PhantomData)
        }
    }

    #[allow(non_snake_case)]
    pub fn make_NULL(self) -> Unique<'a, &'a nodes::A_Const> {
        let mut node = self.make_node::<nodes::A_Const>();
        node.as_mut().into_inner().isnull = true;
        node
    }

    #[allow(non_snake_case)]
    pub fn make_ParamRef(self, number: c_int) -> Unique<'a, &'a nodes::ParamRef> {
        let mut node = self.make_node::<nodes::ParamRef>();
        node.as_mut().into_inner().number = number;
        node
    }

    #[allow(non_snake_case)]
    pub fn make_RawStmt(self, stmt: Unique<'a, Node<'a>>) -> Unique<'a, &'a nodes::RawStmt> {
        let mut raw_stmt = self.make_node::<nodes::RawStmt>();
        raw_stmt.as_mut().set_stmt(stmt);
        raw_stmt
    }

    #[allow(non_snake_case)]
    pub fn make_ResTarget(
        self,
        name: Option<&str>,
        indirection: Unique<'a, &'a NodeList>,
        val: Unique<'a, Node<'a>>,
    ) -> Unique<'a, &'a nodes::ResTarget> {
        let mut res_target = self.make_node::<nodes::ResTarget>();
        if let Some(name) = name {
            res_target.as_mut().into_inner().name = self.copy_string(name);
        }
        res_target.as_mut().set_indirection(indirection);
        res_target.as_mut().set_val(val);
        Unique(res_target.into_ptr(), self.id, PhantomData)
    }

    #[allow(non_snake_case)]
    pub fn make_WithClause(
        self,
        ctes: Unique<'a, &'a CastNodeList<nodes::CommonTableExpr>>,
        recursive: bool,
    ) -> Unique<'a, &'a nodes::WithClause> {
        let mut with_clause = self.make_node::<nodes::WithClause>();
        with_clause.as_mut().set_ctes(ctes);
        with_clause.as_mut().into_inner().recursive = recursive;
        with_clause
    }

    /// Performs a deep copy of the given node onto this memory context,
    /// returning a unique pointer to it.
    pub fn make_unique<T: AsNodePtr>(self, node: T) -> Unique<'a, T::ConvertLifetime<'a>> {
        let node_ptr = node.as_ptr();
        let mut err = ptr::null_mut();
        // SAFETY: This never panics
        let copied = unsafe {
            self.mem
                .within(|| raw::wrapped_copy_object(node_ptr, &mut err))
        };
        if !err.is_null() {
            panic!("Unable to copy node of type {}", type_name::<T>())
        }

        Unique(copied.cast(), self.id, PhantomData)
    }

    /// Create a new instance of the given Node type. The entire allocation
    /// will be zeroed, meaning all pointers will be None, all lists will be
    /// empty, and all primitives will be 0
    ///
    /// The return value of this may not be a logically valid instance of a
    /// node, but it will be semantically valid and unsafe code must ensure it
    /// does not result in undefined behavior if it receives such an instance
    pub fn make_node<T>(self) -> Unique<'a, &'a T>
    where
        T: ConstructableNode,
    {
        let size = std::mem::size_of::<T>();
        let tag = T::TAG;
        // SAFETY: This never panics
        let ptr = unsafe { self.mem.within(|| raw::newNode(size, tag)) };
        Unique(ptr.cast(), self.id, PhantomData)
    }

    pub(crate) fn copy_string(self, s: &str) -> *mut c_char {
        // SAFETY: This never panics
        unsafe {
            self.mem
                .within(|| raw::wrapped_pnstrdup(s.as_ptr().cast(), s.len()))
        }
    }

    /// Returns an empty list
    pub fn empty(self) -> Unique<'a, &'a NodeList> {
        Unique(ptr::null_mut(), self.id, PhantomData)
    }

    /// Returns a NULL pointer to a node (a.k.a. None)
    pub fn none(self) -> Unique<'a, Node<'a>> {
        Unique(ptr::null_mut(), self.id, PhantomData)
    }
}

/// A uniquely owned pointer to a node. This is effectively `Box<T>`, but
/// constrained to the lifetime of its memory context.
#[repr(C)]
pub struct Unique<'a, T>(*mut raw::Node, Id<'a>, PhantomData<T>);

impl<'a, T> Unique<'a, T> {
    /// Consume this to get the inner raw node pointer, erasing its lifetime.
    /// The returned pointer should either be stored along side the memory
    /// context, or assigned to the field of a node within the same memory
    /// context.
    pub(crate) fn into_ptr(self) -> *mut raw::Node {
        self.0
    }

    /// Erase the concrete type, returning a unique [`Node<'a>`]
    pub fn uncast(self) -> Unique<'a, Node<'a>> {
        Unique(self.0, self.1, PhantomData)
    }

    #[cfg(test)]
    fn into_inner(self) -> T
    where
        T: crate::FromNodePtr,
    {
        // SAFETY: Always a valid pointer
        unsafe { T::from_raw(self.into_ptr()) }
    }
}

impl<'a, T> Unique<'a, &'a T>
where
    T: FromNodeMut,
{
    /// Get a mutable reference to the inner node, preventing any assignments
    /// that would mix memory contexts. Panics if called on a null pointer
    ///
    /// ```compile_fail
    /// use pg_raw_parse::make::*;
    /// use pg_raw_parse::nodes::A_Expr_Kind::*;
    /// use pg_raw_parse::Node;
    ///
    /// owned(|mem| {
    ///     let mut expr = mem.make_A_Expr(
    ///         AEXPR_OP,
    ///         mem.empty(),
    ///         mem.none(),
    ///         mem.none(),
    ///     );
    ///     owned(|mem2| {
    ///         expr.as_mut().set_lexpr(mem2.make_String(Some("oops")).uncast());
    ///         mem2.make_String(Some("lol"))
    ///     });
    ///     expr
    /// });
    /// ```
    ///
    /// ```
    /// use pg_raw_parse::make::*;
    /// use pg_raw_parse::nodes::A_Expr_Kind::*;
    /// use pg_raw_parse::Node;
    ///
    /// let expr = owned(|mem| {
    ///     let mut expr = mem.make_A_Expr(
    ///         AEXPR_OP,
    ///         mem.empty(),
    ///         mem.none(),
    ///         mem.none(),
    ///     );
    ///     expr.as_mut().set_lexpr(mem.make_String(Some("lexpr")).uncast());
    ///     expr
    /// });
    /// assert_eq!(Some("lexpr"), expr.lexpr().as_str());
    /// std::assert_matches!(expr.rexpr(), Node::None);
    /// ```
    pub fn as_mut<'b>(&'b mut self) -> T::MutRef<'a, 'b> {
        let ptr = NonNull::new(self.0)
            .expect("as_mut called on a NULL pointer")
            .cast();
        // SAFETY: This was always constructed with a valid pointer
        unsafe { T::from_ptr_mut(ptr, self.1) }
    }
}

/// Construct an owned node. A new memory context will be created, and passed
/// to the given function to allocate onto it. The entire arena will be owned
/// by the return value of this function. The given closure may only return data
/// owned by the memory context passed as an argument
///
/// ```compile_fail
/// use pg_raw_parse::make::owned;
///
/// let mut node = None;
/// owned(|mem| {
///     node = Some(mem.make_String(Some("smuggled")));
///     mem.make_String(Some("returned"))
/// });
/// ```
///
/// ```compile_fail
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     owned(|mem2| mem1.make_String(Some("wrong mem")));
///     mem1.make_String(Some("right mem"))
/// });
/// ```
pub fn owned<F, T>(f: F) -> Owned<T>
where
    for<'a> F: FnOnce(MemoryToken<'a>) -> Unique<'a, &'a T>,
{
    let mem = MemoryContext::new(c"pg_raw_parse_owned_node");
    let node = {
        generativity::make_guard!(a);
        let token = MemoryToken {
            mem: &mem,
            id: a.into(),
        };
        f(token).into_ptr()
    };
    Owned::new(mem, node)
}

#[test]
fn make_empty_list() {
    let list = owned(|mem| mem.empty());
    assert!(list.as_ptr().is_null());
}

#[test]
fn copy_null_pointer() {
    let none_node = Node::None;
    let empty_list = &crate::list::EMPTY_LIST;

    let copy_list = owned(|mem| {
        let copy_none = mem.make_unique(none_node);
        assert!(copy_none.into_ptr().is_null());
        let copy_option_none = mem.make_unique(None::<&nodes::RangeVar>);
        assert!(copy_option_none.into_ptr().is_null());
        mem.make_unique(empty_list)
    });
    assert!(copy_list.as_ptr().is_null());
}

#[test]
fn copy_node() {
    let s = owned(|mem| mem.make_String(Some("hi")));
    let copied_string = owned(|mem| {
        let copied_node = mem.make_unique(Node::String(&*s));
        assert_eq!(Some("hi"), copied_node.into_inner().as_str());
        mem.make_unique(&*s)
    });
    assert_eq!(Some("hi"), copied_string.sval());
}

#[test]
fn make_complex_node() {
    use crate::nodes::A_Expr_Kind;

    let a_expr = owned(|mem| {
        mem.make_A_Expr(
            A_Expr_Kind::AEXPR_OP,
            mem.make_List(&[mem.make_String(Some("=")).uncast()]),
            mem.make_ColumnRef(mem.make_List(&[mem.make_String(Some("id")).uncast()]))
                .uncast(),
            mem.make_A_Const(ConstValue::Integer(1)).uncast(),
        )
    });

    std::assert_matches!(
        &*a_expr,
        nodes::A_Expr {
            kind: A_Expr_Kind::AEXPR_OP,
            ..
        } if a_expr.name().iter().map(Node::as_str).eq([Some("=")])
            && matches!(a_expr.lexpr(), Node::ColumnRef(c)
                if c.fields().iter().map(Node::as_str).eq([Some("id")]))
            && matches!(a_expr.rexpr(), Node::A_Const(c)
                if c.val().and_then(|c| c.numeric_value::<i32>()) == Some(1))
    );
}

#[test]
fn make_node_with_cast_list() {
    let stmt = owned(|mem| {
        let mut select_stmt = mem.make_node::<nodes::SelectStmt>();
        let list = mem.make_List(&[mem.make_ResTarget(
            None,
            mem.empty(),
            mem.make_A_Const(ConstValue::Integer(1)).uncast(),
        )]);
        select_stmt.as_mut().set_targetList(list);
        mem.make_RawStmt(select_stmt.uncast())
    });

    assert_eq!("SELECT 1", crate::deparse(&stmt).unwrap().as_str());
}
