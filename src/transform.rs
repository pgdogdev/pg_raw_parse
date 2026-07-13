use crate::list::{CastNodeList, NodeList};
use crate::list_mut::NodeListMut;
use crate::make::Unique;
use crate::{Node, NodeMut, nodes};
use generativity::Id;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

include!(concat!(env!("OUT_DIR"), "/transform_raw.rs"));

/// A [`NodeMut`] that can be replaced with another node of any type. This type
/// exists to prevent taking a field of a specific type (such as
/// [`nodes::InsertStmt::relation`], upcasting to [`NodeMut`] and assigning a
/// different type.
///
/// ```compile_fail
/// use pg_raw_parse::transform::*;
/// use pg_raw_parse::*;
///
/// make::owned(|mem| {
///     let mut insert_stmt = mem.make_node::<nodes::InsertStmt>();
///     let mut string = mem.make_string(Some("not a RangeVar")).uncast();
///     let mut insert_stmt_mut = insert_stmt.as_mut();
///     let mut range_var_mut = NodeMut::RangeVar(insert_stmt_mut.relation_mut().unwrap());
///
///     transform(&mut string, |node| match &*node {
///         NodeMut::String(s) => {
///             // Smuggle the RangeVarMut into the Assignable
///             std::mem::swap(&mut *node, &mut range_var_mut);
///             // Replace the RangeVar with a String. This would be unsound
///             // if it compiled
///             node.replace(mem.make_string(Some("oops")).uncast());
///             None
///         }
///         _ => Some(node)
///     });
///
///     insert_stmt
/// });
/// ```
pub struct Assignable<'mem, 'mutref>(NodeMut<'mem, 'mutref>);

impl<'mem, 'mutref> Assignable<'mem, 'mutref> {
    pub fn new(node: &'mutref mut Unique<'mem, Node<'_>>) -> Self {
        Self(node.as_mut())
    }

    pub fn into_inner(self) -> NodeMut<'mem, 'mutref> {
        self.0
    }
}

impl<'mem, 'mutref> Deref for Assignable<'mem, 'mutref> {
    type Target = NodeMut<'mem, 'mutref>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'mem, 'mutref> DerefMut for Assignable<'mem, 'mutref> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'mem, 'mutref> Assignable<'mem, 'mutref> {
    pub fn replace(self, node: Unique<'mem, Node<'_>>) {
        // SAFETY: It is impossible to construct Assignable elsewhere. We
        // only construct this with generic nodes
        unsafe { self.0.replace(node) };
    }
}

pub struct TransformClosure<'mem, F>(F, PhantomData<Id<'mem>>);

impl<'mem, F> TransformClosure<'mem, F>
where
    F: for<'mutref> FnMut(Assignable<'mem, 'mutref>) -> Option<Assignable<'mem, 'mutref>>,
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<'mem, F> Transform<'mem> for TransformClosure<'mem, F>
where
    F: for<'mutref> FnMut(Assignable<'mem, 'mutref>) -> Option<Assignable<'mem, 'mutref>>,
{
    fn transform_node<'mutref>(&mut self, node: Assignable<'mem, 'mutref>) {
        if let Some(node) = self.0(node) {
            transform_node(node.0, self)
        }
    }
}

/// Transforms all transformable nodes using the given function. Return the
/// input node for any nodes you'd like to recurse through. Return `None` for
/// any nodes you've replaced, or that you don't wish to recurse through
///
/// Note that this will only call the callback for nodes that can be replaced
/// with a node of any type. It will not be called for any lists of a known
/// type, or fields of a known type, as replacing those with a node of a
/// different type will result in undefined behavior. To be able to transform
/// nodes of a specific, known type, implement [`Transform`] directly, or use
/// [`walk_mut`](crate::walk::walk_mut).
pub fn transform<'mem, F>(node: &mut Unique<'mem, Node<'_>>, f: F)
where
    F: for<'mutref> FnMut(Assignable<'mem, 'mutref>) -> Option<Assignable<'mem, 'mutref>>,
{
    TransformClosure::new(f).transform_node(Assignable::new(node));
}

#[test]
fn transform_arbitrary_nodes() {
    use crate::raw::A_Expr_Kind;
    use crate::{ConstValue, Node, deparse, make, parse};

    let tree = parse(
        "SELECT unique_id(), (SELECT unique_id() FROM users WHERE id = 2), less_unique_id() FROM users WHERE id = baz(3) AND qux = unique_id()",
    ).unwrap();
    let stmt = tree.into_iter().next().unwrap();

    // Replace all calls to `unique_id()` with an incrementing counter, and any
    // comparisons based on a column called `id` with `true`
    let replaced = make::owned(|mem| {
        let mut copy = mem.make_unique(stmt);
        let mut id = 0;
        TransformClosure::new(|node| match &*node {
            NodeMut::FuncCall(f)
                if f.funcname()
                    .iter()
                    .filter_map(Node::as_str)
                    .eq(["unique_id"]) =>
            {
                node.replace(mem.make_a_const(ConstValue::Integer(id)).uncast());
                id += 1;
                None
            }
            NodeMut::A_Expr(expr)
                if expr.kind == A_Expr_Kind::AEXPR_OP
                    && expr.name().iter().filter_map(Node::as_str).eq(["="])
                    && let Node::ColumnRef(c) = expr.lexpr()
                    && c.fields().iter().filter_map(Node::as_str).eq(["id"]) =>
            {
                node.replace(mem.make_a_const(ConstValue::Boolean(true)).uncast());
                None
            }
            _ => Some(node),
        })
        .transform_raw_stmt(copy.as_mut());

        copy
    });

    assert_eq!(
        deparse(&*replaced).unwrap().as_str(),
        "SELECT 0, (SELECT 1 FROM users WHERE true), less_unique_id() FROM users WHERE true AND qux = 2",
    );
}

#[test]
fn replace_nodes_of_specific_type() {
    let tree = crate::parse(
        "WITH stuff AS (INSERT INTO qux DEFAULT VALUES) SELECT (SELECT * FROM bar), * FROM foo",
    )
    .unwrap();
    let stmt = tree.into_iter().next().unwrap();

    let replaced = crate::make::owned(|mem| {
        let mut copy = mem.make_unique(stmt);

        struct ReplaceAllTableNames<'mem>(crate::make::MemoryToken<'mem>);

        impl<'mem> Transform<'mem> for ReplaceAllTableNames<'mem> {
            fn transform_range_var<'mutref>(&mut self, node: nodes::RangeVarMut<'mem, 'mutref>) {
                node.replace(self.0.make_range_var(None, Some("baz")));
            }
        }

        ReplaceAllTableNames(mem).transform_raw_stmt(copy.as_mut());

        copy
    });

    assert_eq!(
        crate::deparse(&*replaced).unwrap().as_str(),
        "WITH stuff AS (INSERT INTO baz DEFAULT VALUES) SELECT (SELECT * FROM baz), * FROM baz",
    );
}
