// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

/// The `toposort` function sorts `nodes` in a topological order.
///
/// The topological constraints are provided by the `deps` parameter.  Each
/// `Dependency` provides a requirement that a `predecessor` has to appear
/// before a `successor`.
///
/// If `deps` form a cycle, then the result is split into:
/// - topologically `ordered` nodes,
/// - `failed` nodes (which either form dependency cycles, or depend on a
///   cycle).
///
/// If there are multiple possible topological orders, then best effort is made
/// to return the `ordered` notes in the `preferred_order` (this is not always
/// possible given the constraints specified by `deps`).  `failed` nodes are
/// returned sorted in the `preferred_order`.
///
/// The `toposort` function is generic and can theoretically work with an
/// arbitrary type of graph nodes. In practice the node type should be a cheaply
/// `Clone`able `NodeId` (rather than a full node). Decoding `Dependency`
/// information additionally requires that the `NodeId` implements
/// `Eq` and `Hash`.
///
/// # Example
///
/// ```
/// let TopoSortResult { ordered, failed } = toposort(
///     vec![101, 102, 103, 104, 201, 202, 203, 204, 901, 902, 903],
///     vec![
///         Dependency { predecessor: 102, successor: 103 },
///         Dependency { predecessor: 203, successor: 202 },
///         Dependency { predecessor: 901, successor: 902 },
///         Dependency { predecessor: 902, successor: 901 },
///         Dependency { predecessor: 901, successor: 903 },
///     ],
///     Ord::cmp,
/// );
/// assert_eq!(ordered, vec![101, 102, 103, 104, 201, 203, 202, 204]);
/// assert_eq!(failed, vec![901, 902, 903]);
/// ```
///
/// In the example above:
/// - Nodes 101-104 are returned in the preferred order, because the 102 -> 103
///   dependency edge is compatible with the preferred order.
/// - Nodes 201-204 have to be reordered slightly because of the 203 -> 202
///   dependency edge.
/// - Nodes 901 and 902 form a cycle.  Node 903 depends on the a node from the
///   cycle.
///
/// # Implementation details
///
/// The function below implements [the Kahn's
/// algorithm](https://en.wikipedia.org/wiki/Topological_sorting#Kahn%27_algorithm), but the `S`
/// structure from the algorithm is implemented using [a priority
/// queue](https://en.wikipedia.org/wiki/Priority_queue) - this helps remove nodes in the desired
/// order.
///
/// Rust's standard library conveniently provides a priority queue as
/// `std::collections::BinaryHeap`.  The `preferred_order` function is exposed
/// to the `BinaryHeap` via the `Ord` implementation of the private `HeapNode`
/// struct below.
///
/// # Why not use an existing Cargo crate?
///
/// There are existing Cargo crate that provide an implementation of the
/// topological sort:
/// - https://docs.rs/topological-sort
/// - https://docs.rs/petgraph (`petgraph::algo::toposort`)
///
/// We aren't using these Cargo crates, because they don't meet all of our
/// requirements - see below for details.
///
/// ## Best effort sorting and reporting cycle details
///
/// In cases where topological sorting of *all* nodes is not possible Crubit
/// doesn't want to just give up - it wants to:
/// - Topologically sort as many nodes as possible (and emit at least _their_
///   C++ bindings)
/// - Identify nodes that form cycles or depend on cycles (to emit diagnostics
///   explaining why no C++ bindings have been generated for APIs associated
///   with these nodes)
///
/// `petgraph::algo::toposort` returns a `Result<..., Cycle<G::NodeId>`.  When
/// an error is reported:
/// - The sortable subset of nodes is not returned
/// - Only a single cycle-forming node is reported
///
/// `topological_sort` crate supports detecting a cycle (`pop` returns `None`
/// when `len()` is still non-zero), but it doesn't support identifying the
/// remaining nodes (ones that contain a cycle).
///
/// ## Best effort to preserve the preferred order
///
/// If possible, then Crubit wants to emit the C++ bindings in the same order as
/// the order in which the wrapped Rust APIs are present in the `.rs` sources.
/// Preserving this order is not always possible (because of dependencies
/// between the generated C++ bindings), but Crubit wants to make the best
/// effort to preserve this order.  In particular, when the preferred order
/// already *is* a topological order, then it should be kept.
///
/// `petgraph::algo::toposort` doesn't support extra ordering that can be used
/// to decide the order between nodes that don't have a direct or indirect
/// topological dependency.
///
/// `topological_sort` can return results in chunks, so items in these chunks
/// can potentially be sorted before further processing.  Unfortunately this
/// approach isn't sufficient in some scenarios - consider the example below
/// (this scenario is covered by the
/// `test_toposort_deps_compatible_with_preferred_order` unit test):
///
///     ```
///     use topological_sort::TopologicalSort;
///     let mut sorter = TopologicalSort::new();
///     sorter.insert(1);
///     sorter.insert(2);
///     sorter.insert(3);
///     sorter.insert(4);
///     sorter.add_dependency(2, 3);
///
///     let mut ordered = vec![];
///     loop {
///         let ordered_chunk = sorter.pop_all();
///         ordered_chunk.sort();
///         ordered.extend(ordered_chunk);
///         if orderer_chunk.is_empty() {
///             break;
///         }
///     }
///     ```
///
/// The first call to `pop_all` would append nodes 1,2,4 to `ordered`.  The
/// second and last call would append node 3.  This would produce
/// 1,2,4,3 order when the preferred order (1,2,3,4) is also a valid
/// topological order.
pub fn toposort<NodeId, CmpFn>(
    nodes: impl IntoIterator<Item = NodeId>,
    deps: impl IntoIterator<Item = Dependency<NodeId>>,
    preferred_order: CmpFn,
) -> TopoSortResult<NodeId>
where
    NodeId: Clone + Eq + Hash,
    CmpFn: Fn(&NodeId, &NodeId) -> Ordering,
{
    // Translating `nodes` and `deps` into a `graph` that maps node ids into 1)
    // `count_of_predecessors` and 2) a set of `successor` ids.
    let mut graph: HashMap<NodeId, GraphNode<NodeId>> =
        nodes.into_iter().map(|id| (id, GraphNode::default())).collect();
    for Dependency { predecessor, successor } in deps.into_iter() {
        graph
            .get_mut(&successor)
            .expect("`Dependency::successor` should refer to a NodeId in the `nodes` parameter")
            .count_of_predecessors += 1;
        graph
            .get_mut(&predecessor)
            .expect("`Dependency::predecessor` should refer to a NodeId in the `nodes` parameter")
            .successors
            .push(successor);
    }

    // `ready` contains ids of nodes which have no remaining predecessors (and which
    // therefore are ready to be added to the `ordered` result of the
    // topological sort).  Using a BinaryHeap to store the `ready` nodes helps
    // to extract them in the `preferred_order`.  (This is the `S` data structure from
    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn%27s_algorithm.)
    let mut ready: BinaryHeap<HeapNode<'_, NodeId, CmpFn>> = graph
        .iter()
        .filter(|(_, graph_node)| graph_node.count_of_predecessors == 0)
        .map(|(id, _)| HeapNode { id: id.clone(), cmp_fn: &preferred_order })
        .collect();

    // `ordered` contains the topologically ordered results.  (This is the `L` list
    // from https://en.wikipedia.org/wiki/Topological_sorting#Kahn%27s_algorithm.)
    let mut ordered: Vec<NodeId> = Vec::with_capacity(graph.len());
    while let Some(HeapNode { id: removed_id, .. }) = ready.pop() {
        let removed_graph_node = graph.remove(&removed_id).unwrap();
        for succ_id in removed_graph_node.successors.into_iter() {
            let succ = graph.get_mut(&succ_id).unwrap();
            assert!(succ.count_of_predecessors > 0);
            succ.count_of_predecessors -= 1;
            if succ.count_of_predecessors == 0 {
                ready.push(HeapNode { id: succ_id, cmp_fn: &preferred_order });
            }
        }
        ordered.push(removed_id);
    }

    // `failed` contains the remaining nodes - ones that either formed a dependency
    // cycle or (possibly indirectly) depended on a node participating in a
    // cycle.
    let mut failed: Vec<NodeId> = graph.into_keys().collect();
    failed.sort_by(preferred_order);

    TopoSortResult { ordered, failed }
}

/// Topological ordering dependency between two graph nodes.  The `predecessor`
/// has to appear before the `successor` in `TopoSortResult::ordered`.  See the
/// `toposort` function for more details.
pub struct Dependency<NodeId> {
    pub predecessor: NodeId,
    pub successor: NodeId,
}

/// Result of a topological sort.  See the `toposort` function for more details.
pub struct TopoSortResult<NodeId> {
    /// Topologically `ordered` graph nodes,
    pub ordered: Vec<NodeId>,

    /// `failed` graph nodes - ones that either form dependency cycles, or
    /// (directly, or transitively) depend on a node that participates in a
    /// cycle.
    pub failed: Vec<NodeId>,
}

struct GraphNode<NodeId> {
    count_of_predecessors: usize,
    successors: Vec<NodeId>,
}

// TODO(https://github.com/rust-lang/rust/issues/26925): It should be possible to
// `#[derive(Default)]` here, but `derive` insists that `NodeId` has to
// implement `Default` even though the default vector doesn't contain any
// `NodeId`s.
impl<NodeId> Default for GraphNode<NodeId> {
    fn default() -> Self {
        Self { count_of_predecessors: 0, successors: Vec::new() }
    }
}

struct HeapNode<'a, NodeId, CmpFn>
where
    CmpFn: Fn(&NodeId, &NodeId) -> Ordering,
{
    id: NodeId,
    cmp_fn: &'a CmpFn,
}

impl<'a, NodeId, CmpFn> Ord for HeapNode<'a, NodeId, CmpFn>
where
    CmpFn: Fn(&NodeId, &NodeId) -> Ordering,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.pop
        // "removes the greatest item from the binary heap" and therefore to pop items
        // in the `cmp_fn`-described preferred order we need to call `reverse()`
        // below.
        (self.cmp_fn)(&self.id, &other.id).reverse()
    }
}

impl<'a, NodeId, CmpFn> PartialOrd for HeapNode<'a, NodeId, CmpFn>
where
    CmpFn: Fn(&NodeId, &NodeId) -> Ordering,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, NodeId, CmpFn> PartialEq for HeapNode<'a, NodeId, CmpFn>
where
    CmpFn: Fn(&NodeId, &NodeId) -> Ordering,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a, NodeId, CmpFn> Eq for HeapNode<'a, NodeId, CmpFn> where
    CmpFn: Fn(&NodeId, &NodeId) -> Ordering
{
}

#[cfg(test)]
mod tests {
    /// Test helper providing simplified API for `super::toposort`:
    /// - `NodeId`s are integers
    /// - The `preferred_order` is the natural order of integers
    /// - `nodes` and `deps` are slices (rather than `IntoIterator<...>`).
    /// - To avoid boilerplate and improve readability of tests `Dependency` and
    ///   `TopoSortResult` are replaced with tuples:
    ///     - `Dependency{ predecessor, successor }` => `(predecessor,
    ///       successor)`
    ///     - `TopoSortResult{ ordered, failed }` => (ordered, failed)`
    fn toposort(nodes: &[i32], deps: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
        let nodes = nodes.iter().copied();
        let deps = deps
            .iter()
            .copied()
            .map(|(predecessor, successor)| super::Dependency { predecessor, successor });
        let result = super::toposort(nodes, deps, Ord::cmp);
        (result.ordered, result.failed)
    }

    #[test]
    fn test_toposort_empty() {
        let (ordered, failed) = toposort(&[], &[]);
        assert_eq!(ordered, vec![]);
        assert_eq!(failed, vec![]);
    }

    #[test]
    fn test_toposort_no_deps() {
        let (ordered, failed) = toposort(&[1, 4, 2, 3], &[]);
        assert_eq!(ordered, vec![1, 2, 3, 4]);
        assert_eq!(failed, vec![]);
    }

    #[test]
    fn test_toposort_deps_compatible_with_preferred_order() {
        let (ordered, failed) = toposort(&[1, 4, 2, 3], &[(2, 3)]);
        assert_eq!(ordered, vec![1, 2, 3, 4]);
        assert_eq!(failed, vec![]);
    }

    #[test]
    fn test_toposort_deps_incompatible_with_preferred_order() {
        let (ordered, failed) = toposort(&[1, 4, 2, 3], &[(3, 2)]);
        assert_eq!(ordered, vec![1, 3, 2, 4]);
        assert_eq!(failed, vec![]);
    }

    #[test]
    fn test_toposort_cycle() {
        let (ordered, failed) = toposort(&[1, 2, 3, 4, 5, 6, 7], &[(3, 5), (5, 6), (6, 5), (5, 7)]);
        assert_eq!(ordered, vec![1, 2, 3, 4]);
        assert_eq!(failed, vec![5, 6, 7]);
    }

    #[test]
    fn test_example() {
        // TODO: Remove this test once rustdoc examples of the `toposort` function are
        // directly tested (currently Bazel doesn't support running rustdoc
        // examples as tests).
        use super::{toposort, Dependency, TopoSortResult};
        let TopoSortResult { ordered, failed } = toposort(
            vec![101, 102, 103, 104, 201, 202, 203, 204, 901, 902, 903],
            vec![
                Dependency { predecessor: 102, successor: 103 },
                Dependency { predecessor: 203, successor: 202 },
                Dependency { predecessor: 901, successor: 902 },
                Dependency { predecessor: 902, successor: 901 },
                Dependency { predecessor: 901, successor: 903 },
            ],
            Ord::cmp,
        );
        assert_eq!(ordered, vec![101, 102, 103, 104, 201, 203, 202, 204]);
        assert_eq!(failed, vec![901, 902, 903]);
    }
}
