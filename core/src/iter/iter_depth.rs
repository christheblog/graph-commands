///! Graph Iterator implementation
use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::iter::iter_datastructure::{SearchQueue, Stack};

use std::collections::HashSet;


///! Depth-First search iterator

pub struct DepthFirstIter<'a> {
    stack: Stack<VertexId>,
    visited: HashSet<VertexId>,
    graph: &'a DirectedGraph,
}

impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = VertexId;
    fn next(&mut self) -> Option<VertexId> {
        match self.stack.pop() {
            None => None,
            Some(vid) => {
                self.graph
                    .outbound_edges(vid)
                    .map(|Edge(_, v)| v)
                    .for_each(|v| {
                        if !self.visited.contains(v) {
                            self.visited.insert(*v);
                            self.stack.push(*v)
                        }
                    });
                Some(vid)
            }
        }
    }
}

/// Returns a new depth first search iterator on the given graph
pub fn dfs_iter(graph: &DirectedGraph) -> DepthFirstIter {
    match graph.head_option() {
        None => empty_dfs_iter(graph),
        Some(head) => dfs_iter_from(graph, *head),
    }
}

/// Returns a new depth first search iterator on the given graph, starting from the given start_vertex
pub fn dfs_iter_from(graph: &DirectedGraph, start_vertex: VertexId) -> DepthFirstIter {
    let mut dfs_iter = empty_dfs_iter(graph);
    dfs_iter.stack.push(start_vertex);
    dfs_iter.visited.insert(start_vertex);
    dfs_iter
}

/// Builds an empty iterator from a given graph.
fn empty_dfs_iter(graph: &DirectedGraph) -> DepthFirstIter {
    DepthFirstIter {
        stack: Stack::<VertexId>::new(),
        visited: HashSet::new(),
        graph: graph,
    }
}
