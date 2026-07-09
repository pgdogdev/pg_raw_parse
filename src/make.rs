use crate::list::{CastNodeList, NodeList};
use crate::mem::MemoryContext;
use crate::raw::{self, *};
use crate::{
    AsNodePtr, ConstValue, ConstructableNode, FromNodeMut, FromNodePtr, Node, Owned, nodes,
};
use generativity::Id;
use std::any::type_name;
use std::ffi::{c_char, c_int};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr;

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
///     let node = mem1.make_string(Some("hi"));
///     owned(|mem2| mem2.make_list(&[node])); // Fails, node is on mem1
///     mem1.make_string(Some("lol"))
/// });
/// ```
///
/// ```
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     let node = mem1.make_string(Some("hi"));
///     mem1.make_list(&[node]) // Is fine, both nodes are on mem1
/// });
/// ```
#[derive(Clone, Copy)]
pub struct MemoryToken<'mem> {
    mem: &'mem MemoryContext,
    id: Id<'mem>,
}

impl<'mem> MemoryToken<'mem> {
    pub fn make_a_const(self, val: ConstValue<'mem>) -> Unique<'mem, &'mem nodes::A_Const> {
        let mut node = self.make_node::<nodes::A_Const>();
        node.as_mut().set_isnull(false);
        node.as_mut().set_val(val.as_raw(self));
        node
    }

    pub fn make_column_ref(
        self,
        fields: Unique<'mem, &'mem NodeList>,
    ) -> Unique<'mem, &'mem nodes::ColumnRef> {
        let mut node = self.make_node::<nodes::ColumnRef>();
        node.as_mut().set_fields(fields);
        node
    }

    pub fn make_common_table_expr(
        self,
        ctename: &str,
        aliascolnames: Unique<'mem, &'mem NodeList>,
        ctequery: Unique<'mem, Node<'mem>>,
    ) -> Unique<'mem, &'mem nodes::CommonTableExpr> {
        let mut cte = self.make_node::<nodes::CommonTableExpr>();
        cte.as_mut().set_ctename(Some(self.copy_string(ctename)));
        cte.as_mut().set_aliascolnames(aliascolnames);
        cte.as_mut().set_ctequery(ctequery);
        cte
    }

    pub fn make_list<T: AsNodePtr>(self, elems: &[Unique<'mem, T>]) -> Unique<'mem, &'mem T::List> {
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

    pub fn make_null(self) -> Unique<'mem, &'mem nodes::A_Const> {
        let mut node = self.make_node::<nodes::A_Const>();
        node.as_mut().set_isnull(true);
        node
    }

    pub fn make_param_ref(self, number: c_int) -> Unique<'mem, &'mem nodes::ParamRef> {
        let mut node = self.make_node::<nodes::ParamRef>();
        node.as_mut().set_number(number);
        node
    }

    pub fn make_raw_stmt(
        self,
        stmt: Unique<'mem, Node<'mem>>,
    ) -> Unique<'mem, &'mem nodes::RawStmt> {
        let mut raw_stmt = self.make_node::<nodes::RawStmt>();
        raw_stmt.as_mut().set_stmt(stmt);
        raw_stmt
    }

    pub fn make_res_target(
        self,
        name: Option<&str>,
        indirection: Unique<'mem, &'mem NodeList>,
        val: Unique<'mem, Node<'mem>>,
    ) -> Unique<'mem, &'mem nodes::ResTarget> {
        let mut res_target = self.make_node::<nodes::ResTarget>();
        res_target
            .as_mut()
            .set_name(name.map(|n| self.copy_string(n)));
        res_target.as_mut().set_indirection(indirection);
        res_target.as_mut().set_val(val);
        Unique(res_target.into_ptr(), self.id, PhantomData)
    }

    pub fn make_with_clause(
        self,
        ctes: Unique<'mem, &'mem CastNodeList<nodes::CommonTableExpr>>,
        recursive: bool,
    ) -> Unique<'mem, &'mem nodes::WithClause> {
        let mut with_clause = self.make_node::<nodes::WithClause>();
        with_clause.as_mut().set_ctes(ctes);
        with_clause.as_mut().set_recursive(recursive);
        with_clause
    }

    /// Performs a deep copy of the given node onto this memory context,
    /// returning a unique pointer to it.
    pub fn make_unique<T: AsNodePtr>(self, node: T) -> Unique<'mem, T::AsRef<'mem>> {
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
    pub fn make_node<T>(self) -> Unique<'mem, &'mem T>
    where
        T: ConstructableNode,
    {
        let size = std::mem::size_of::<T>();
        let tag = T::TAG;
        // SAFETY: This never panics
        let ptr = unsafe { self.mem.within(|| raw::newNode(size, tag)) };
        Unique(ptr.cast(), self.id, PhantomData)
    }

    /// Copy a string onto this memory context
    pub fn copy_string(self, s: &str) -> PgStr<'mem> {
        // SAFETY: This never panics
        let ptr = unsafe {
            self.mem
                .within(|| raw::wrapped_pnstrdup(s.as_ptr().cast(), s.len()))
        };
        PgStr(ptr, self.id)
    }

    /// Returns an empty list
    pub fn empty(self) -> Unique<'mem, &'mem NodeList> {
        Unique(ptr::null_mut(), self.id, PhantomData)
    }

    /// Returns an empty typed list
    pub fn cast_empty<T>(self) -> Unique<'mem, &'mem CastNodeList<T>> {
        Unique(ptr::null_mut(), self.id, PhantomData)
    }

    /// Returns a NULL pointer to a node (a.k.a. None)
    pub fn none(self) -> Unique<'mem, Node<'mem>> {
        Unique(ptr::null_mut(), self.id, PhantomData)
    }

    pub(crate) fn lappend<T: AsNodePtr>(
        self,
        list: *mut raw::List,
        elem: Unique<'mem, T>,
    ) -> Unique<'mem, &'mem T::List> {
        // SAFETY: This never panics
        let new_list = unsafe {
            self.mem
                .within(|| raw::lappend(list, elem.into_ptr().cast()))
        };
        Unique(new_list.cast(), self.id, PhantomData)
    }

    pub(crate) fn list_insert_nth<T: AsNodePtr>(
        self,
        list: *mut raw::List,
        idx: usize,
        elem: Unique<'mem, T>,
    ) -> Unique<'mem, &'mem T::List> {
        // SAFETY: This never panics
        let new_list = unsafe {
            self.mem
                .within(|| raw::list_insert_nth(list, idx as _, elem.into_ptr().cast()))
        };
        Unique(new_list.cast(), self.id, PhantomData)
    }

    pub(crate) fn list_concat<T>(
        self,
        list: *mut raw::List,
        elems: Unique<'mem, T>,
    ) -> Unique<'mem, T> {
        // SAFETY: This never panics
        let new_list = unsafe {
            self.mem
                .within(|| raw::list_concat(list, elems.into_ptr().cast()))
        };
        Unique(new_list.cast(), self.id, PhantomData)
    }
}

/// A uniquely owned pointer to a node. This is effectively `Box<T>`, but
/// constrained to the lifetime of its memory context.
#[repr(C)]
pub struct Unique<'mem, T>(*mut raw::Node, Id<'mem>, PhantomData<T>);

impl<'mem, T> Unique<'mem, T> {
    /// Consume this to get the inner raw node pointer, erasing its lifetime.
    /// The returned pointer should either be stored along side the memory
    /// context, or assigned to the field of a node within the same memory
    /// context.
    pub(crate) fn into_ptr(self) -> *mut raw::Node {
        self.0
    }

    /// Erase the concrete type, returning a unique [`Node<'mem>`]
    pub fn uncast<'a>(self) -> Unique<'mem, Node<'a>> {
        Unique(self.0, self.1, PhantomData)
    }

    pub fn as_ref(&self) -> T
    where
        T: FromNodePtr,
    {
        // SAFETY: Always a valid pointer
        unsafe { T::from_raw(self.0) }
    }

    pub fn as_option(self) -> Unique<'mem, Option<T>> {
        Unique(self.0, self.1, PhantomData)
    }
}

impl<'mem, T: FromNodeMut<'mem>> Unique<'mem, T> {
    /// Get a mutable reference to the inner node, preventing any assignments
    /// that would mix memory contexts. Panics if called on a null pointer
    ///
    /// ```compile_fail
    /// use pg_raw_parse::make::*;
    /// use pg_raw_parse::nodes::A_Expr_Kind::*;
    /// use pg_raw_parse::Node;
    ///
    /// owned(|mem| {
    ///     let mut expr = mem.make_a_expr(
    ///         AEXPR_OP,
    ///         mem.empty(),
    ///         mem.none(),
    ///         mem.none(),
    ///     );
    ///     owned(|mem2| {
    ///         expr.as_mut().set_lexpr(mem2.make_string(Some("oops")).uncast());
    ///         mem2.make_string(Some("lol"))
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
    ///     let mut expr = mem.make_a_expr(
    ///         AEXPR_OP,
    ///         mem.empty(),
    ///         mem.none(),
    ///         mem.none(),
    ///     );
    ///     expr.as_mut().set_lexpr(mem.make_string(Some("lexpr")).uncast());
    ///     expr
    /// });
    /// assert_eq!(Some("lexpr"), expr.lexpr().as_str());
    /// std::assert_matches!(expr.rexpr(), Node::None);
    /// ```
    pub fn as_mut(&mut self) -> T::MutRef<'_> {
        // SAFETY: This was always constructed with a valid pointer
        unsafe { T::from_ptr_mut(&mut self.0, self.1) }
    }
}

impl<'mem, T> Deref for Unique<'mem, &'mem T>
where
    &'mem T: FromNodePtr,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub struct PgStr<'mem>(*mut c_char, Id<'mem>);

impl PgStr<'_> {
    pub(crate) fn into_ptr(self) -> *mut c_char {
        self.0
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
///     node = Some(mem.make_string(Some("smuggled")));
///     mem.make_string(Some("returned"))
/// });
/// ```
///
/// ```compile_fail
/// use pg_raw_parse::make::owned;
///
/// owned(|mem1| {
///     owned(|mem2| mem1.make_string(Some("wrong mem")));
///     mem1.make_string(Some("right mem"))
/// });
/// ```
pub fn owned<F, T>(f: F) -> Owned<T>
where
    for<'mem> F: FnOnce(MemoryToken<'mem>) -> Unique<'mem, &'mem T>,
{
    try_owned(|mem| Ok::<_, ()>(f(mem))).unwrap()
}

/// Identical to [`owned`], but for functions that return `Result`.
pub fn try_owned<F, T, E>(f: F) -> Result<Owned<T>, E>
where
    for<'mem> F: FnOnce(MemoryToken<'mem>) -> Result<Unique<'mem, &'mem T>, E>,
{
    let mem = MemoryContext::new(c"pg_raw_parse_owned_node");
    let node = {
        generativity::make_guard!(a);
        let token = MemoryToken {
            mem: &mem,
            id: a.into(),
        };
        f(token)?.into_ptr()
    };
    Ok(Owned::new(mem, node))
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
    let s = owned(|mem| mem.make_string(Some("hi")));
    let copied_string = owned(|mem| {
        let copied_node = mem.make_unique(Node::String(&*s));
        assert_eq!(Some("hi"), copied_node.as_ref().as_str());
        mem.make_unique(&*s)
    });
    assert_eq!(Some("hi"), copied_string.sval());
}

#[test]
fn make_complex_node() {
    use crate::nodes::A_Expr_Kind;

    let a_expr = owned(|mem| {
        mem.make_a_expr(
            A_Expr_Kind::AEXPR_OP,
            mem.make_list(&[mem.make_string(Some("=")).uncast()]),
            mem.make_column_ref(mem.make_list(&[mem.make_string(Some("id")).uncast()]))
                .uncast(),
            mem.make_a_const(ConstValue::Integer(1)).uncast(),
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
        let list = mem.make_list(&[mem.make_res_target(
            None,
            mem.empty(),
            mem.make_a_const(ConstValue::Integer(1)).uncast(),
        )]);
        select_stmt.as_mut().set_target_list(list);
        select_stmt
    });

    assert_eq!("SELECT 1", crate::deparse(&*stmt).unwrap().as_str());
}
