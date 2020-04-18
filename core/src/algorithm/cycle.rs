use crate::iter::iter_cycle::Cycle;
use crate::iter::iter_cycle;
use crate::directed_graph::DirectedGraph;


/// Find the first cycle at hand
pub fn first(graph: &DirectedGraph) -> Option<Cycle> {
    iter_cycle::cycle_iter(graph).next()
}

/// Count the number of distinct cycles
pub fn count(graph: &DirectedGraph) -> usize {
    iter_cycle::cycle_iter(graph).count()
}

/// Find the first n cycles at hand
pub fn take(graph: &DirectedGraph, n: usize) -> Vec<Cycle> {
    iter_cycle::cycle_iter(graph).take(n).collect()
}

/// Return all cycles from the graph
/// It would be more efficient to use the iterator for that in case of a big graph
pub fn take_all(graph: &DirectedGraph) -> Vec<Cycle> {
    iter_cycle::cycle_iter(graph).collect()
}

/// Compute the length of the shortest cycle
pub fn girth(graph: &DirectedGraph) -> Option<usize> {
    iter_cycle::cycle_iter(graph).map(|c| c.len()).min()
}

/// Compute the length of the shortest cycle
pub fn shortest(graph: &DirectedGraph) -> Option<Cycle> {
    iter_cycle::cycle_iter(graph).min_by_key(|c| c.len())
}

/// Compute the length of the longest cycle
pub fn longest(graph: &DirectedGraph) -> Option<Cycle> {
    iter_cycle::cycle_iter(graph).max_by_key(|c| c.len())
}
