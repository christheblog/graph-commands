///! Graph Iterator implementation
use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::iter::iter_datastructure::{Queue, SearchQueue};

use std::collections::HashSet;

/// Breadth-First search iterator

pub struct BreadthFirstIter<'a> {
    queue: Queue<VertexId>,
    visited: HashSet<VertexId>,
    graph: &'a DirectedGraph,
}

impl<'a> Iterator for BreadthFirstIter<'a> {
    type Item = VertexId;
    fn next(&mut self) -> Option<VertexId> {
        match self.queue.pop() {
            None => None,
            Some(vid) => {
                self.graph
                    .outbound_edges(vid)
                    .map(|Edge(_, v)| v)
                    .for_each(|v| {
                        if !self.visited.contains(v) {
                            self.visited.insert(*v);
                            self.queue.push(*v)
                        }
                    });
                Some(vid)
            }
        }
    }
}

/// Returns a new breadth first search iterator on the given graph
pub fn bfs_iter(graph: &DirectedGraph) -> BreadthFirstIter {
    match graph.head_option() {
        None => empty_bfs_iter(graph),
        Some(head) => bfs_iter_from(graph, *head),
    }
}

/// Returns a new depth first search iterator on the given graph, starting from the given start_vertex
pub fn bfs_iter_from(graph: &DirectedGraph, start_vertex: VertexId) -> BreadthFirstIter {
    let mut iter = empty_bfs_iter(graph);
    iter.queue.push(start_vertex);
    iter.visited.insert(start_vertex);
    iter
}

/// Builds an empty iterator from a given graph.
fn empty_bfs_iter(graph: &DirectedGraph) -> BreadthFirstIter {
    BreadthFirstIter {
        queue: Queue::<VertexId>::new(),
        visited: HashSet::new(),
        graph: graph,
    }
}
