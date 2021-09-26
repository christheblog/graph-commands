///! Depth-First search iterator
use std::collections::HashSet;

use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::iter::iter_datastructure::{SearchQueue, Stack};
use crate::path::Path;

/// Depth-First search iterator
/// Generates vertex as visited by a depth-first search
pub struct DepthFirstIter<'a> {
    stack: Stack<VertexId>,
    visited: HashSet<VertexId>,
    graph: &'a DirectedGraph,
}

impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = VertexId;

    // Note:
    // children are pushed in reverse order in the stack
    // First child to be considered by the DFS will be the last one
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

/// Builds an empty iterator for a given graph.
fn empty_dfs_iter(graph: &DirectedGraph) -> DepthFirstIter {
    DepthFirstIter {
        stack: Stack::<VertexId>::new(),
        visited: HashSet::new(),
        graph: graph,
    }
}

/// Depth-First search iterator, Returning a full path from the first vertex
/// Useful to generate all possible path without cycle from a given vertex
pub struct DepthFirstPathIter<'a> {
    stack: Stack<Path>,
    graph: &'a DirectedGraph,
}

impl<'a> Iterator for DepthFirstPathIter<'a> {
    type Item = Path;

    // Note:
    // children are pushed in reverse order in the stack
    // First child to be considered by the DFS will be the last one
    fn next(&mut self) -> Option<Path> {
        match self.stack.pop() {
            None => None,
            Some(path) => {
                let vid = path
                    .last()
                    .expect("We shouldn't never have any empty path in the stack !");
                self.graph
                    .outbound_edges(*vid)
                    .map(|Edge(_, v)| v)
                    .for_each(|v| {
                        if !path.contains_vertex(v) {
                            self.stack.push(path.append(*v));
                        }
                    });
                Some(path)
            }
        }
    }
}

/// Returns a new depth first search iterator on the given graph, starting from the given start_vertex
pub fn dfs_iter_path_from(graph: &DirectedGraph, start_vertex: VertexId) -> DepthFirstPathIter {
    let mut dfs_iter = empty_dfs_path_iter(graph);
    dfs_iter.stack.push(Path::from(&vec![start_vertex]));
    dfs_iter
}

/// Builds an empty path iterator for a given graph.
fn empty_dfs_path_iter(graph: &DirectedGraph) -> DepthFirstPathIter {
    DepthFirstPathIter {
        stack: Stack::<Path>::new(),
        graph: graph,
    }
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;

    use super::*;

    #[test]
    fn dfs_iterator_on_an_empty_graph_should_be_empty() {
        let g = DirectedGraph::new();
        let mut it = dfs_iter(&g);
        assert![it.next().is_none(), "Iterator should be empty"]
    }

    #[test]
    fn dfs_iterator_on_a_one_node_graph_should_return_one_node() {
        let mut g = DirectedGraph::new();
        g.add_vertex(VertexId(1));
        let mut it = dfs_iter(&g);
        assert_eq![
            it.next(),
            Some(VertexId(1)),
            "Iterator should return the only node"
        ];
        assert![it.next().is_none(), "Iterator should now be empty"]
    }

    #[test]
    fn dfs_iterator_from_on_a_one_node_graph_should_return_the_only_node() {
        let mut g = DirectedGraph::new();
        g.add_vertex(VertexId(1));
        let mut it = dfs_iter_from(&g, VertexId(1));
        assert_eq![
            it.next(),
            Some(VertexId(1)),
            "Iterator should return the only node"
        ];
        assert![it.next().is_none(), "Iterator should now be empty"]
    }

    #[test]
    fn dfs_iterator_return_reachable_nodes_in_a_depth_first_search_order() {
        fn edge_from(src: u64, end: u64) -> Edge {
            Edge(VertexId(src), VertexId(end))
        }

        let mut g = DirectedGraph::new();
        g.add_edge(edge_from(1, 2));
        g.add_edge(edge_from(1, 4));
        g.add_edge(edge_from(2, 3));
        g.add_edge(edge_from(2, 5));
        g.add_edge(edge_from(1, 5));
        g.add_edge(edge_from(4, 5));
        g.add_edge(edge_from(4, 6));
        g.add_edge(edge_from(6, 7));
        g.add_edge(edge_from(7, 2));
        // 8 is NOT reachable from 1
        g.add_edge(edge_from(8, 2));

        // DFS order from vertex 1
        let it = dfs_iter_from(&g, VertexId(1));
        assert_eq![
            it.collect::<Vec<VertexId>>(),
            vec!(
                VertexId(1),
                // Last child from Vertex 1
                VertexId(5),
                // Middle child
                VertexId(4),
                VertexId(6),
                VertexId(7),
                // First child
                VertexId(2),
                VertexId(3)
            ),
            "DFS order is wrong when starting from Vertex 1"
        ];
    }

    #[test]
    fn dfs_iterator_does_not_loop_when_encountering_a_cycle() {
        fn edge_from(src: u64, end: u64) -> Edge {
            Edge(VertexId(src), VertexId(end))
        }

        let mut g = DirectedGraph::new();
        // cycle
        g.add_edge(edge_from(1, 2));
        g.add_edge(edge_from(2, 3));
        g.add_edge(edge_from(3, 4));
        g.add_edge(edge_from(4, 5));
        g.add_edge(edge_from(5, 1));

        let it = dfs_iter_from(&g, VertexId(1));
        assert_eq![
            it.collect::<Vec<VertexId>>().len(),
            5,
            "DFS returned an invalid length"
        ];
    }

    // DFS path

    #[test]
    fn dfs_path_iterator_return_path_for_all_reachable_nodes_in_a_depth_first_search_order() {
        fn edge_from(src: u64, end: u64) -> Edge {
            Edge(VertexId(src), VertexId(end))
        }

        fn path_of(v: Vec<u64>) -> Path {
            Path::from(&v.iter().map(|id| VertexId(*id)).collect())
        }

        let mut g = DirectedGraph::new();
        g.add_edge(edge_from(1, 2));
        g.add_edge(edge_from(1, 4));
        g.add_edge(edge_from(2, 3));
        g.add_edge(edge_from(2, 5));
        g.add_edge(edge_from(1, 5));
        g.add_edge(edge_from(4, 5));
        g.add_edge(edge_from(4, 6));
        g.add_edge(edge_from(6, 7));
        g.add_edge(edge_from(7, 2));
        // 8 is NOT reachable from 1
        g.add_edge(edge_from(8, 2));

        let it = dfs_iter_path_from(&g, VertexId(1));
        assert_eq![
            it.collect::<Vec<Path>>(),
            vec!(
                path_of(vec![1]),
                // Last child from Vertex 1
                path_of(vec![1, 5]),
                // Middle child
                path_of(vec![1, 4]),
                path_of(vec![1, 4, 6]),
                path_of(vec![1, 4, 6, 7]),
                path_of(vec![1, 4, 6, 7, 2]),
                path_of(vec![1, 4, 6, 7, 2, 5]),
                path_of(vec![1, 4, 6, 7, 2, 3]),
                path_of(vec![1, 4, 5]),
                // First child
                path_of(vec![1, 2]),
                path_of(vec![1, 2, 5]),
                path_of(vec![1, 2, 3])
            ),
            "DFS order is wrong when starting from Vertex 1"
        ];
    }
}
