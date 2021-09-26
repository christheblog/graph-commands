///! Breadth-First search Iterator
use std::collections::HashSet;

use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::iter::iter_datastructure::{Queue, SearchQueue};
use crate::path::Path;

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

/// Returns a new breadth first search iterator on the given graph, starting from the given start_vertex
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

/// Breadth-First Path iterator

pub struct BreadthFirstPathIter<'a> {
    queue: Queue<Path>,
    graph: &'a DirectedGraph,
}

impl<'a> Iterator for BreadthFirstPathIter<'a> {
    type Item = Path;
    fn next(&mut self) -> Option<Path> {
        match self.queue.pop() {
            None => None,
            Some(path) => {
                let vid = path
                    .last()
                    .expect("We shouldn't never have any empty path in the queue !");
                self.graph
                    .outbound_edges(*vid)
                    .map(|Edge(_, v)| v)
                    .for_each(|v| {
                        if !path.contains_vertex(v) {
                            self.queue.push(path.append(*v))
                        }
                    });
                Some(path)
            }
        }
    }
}

/// Returns a new breadth first search iterator on the given graph, starting from the given start_vertex
pub fn bfs_path_iter_from(graph: &DirectedGraph, start_vertex: VertexId) -> BreadthFirstPathIter {
    let mut iter = empty_bfs_path_iter(graph);
    iter.queue.push(Path::from(&vec![start_vertex]));
    iter
}

/// Builds an empty iterator from a given graph.
fn empty_bfs_path_iter(graph: &DirectedGraph) -> BreadthFirstPathIter {
    BreadthFirstPathIter {
        queue: Queue::<Path>::new(),
        graph: graph,
    }
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;

    use super::*;

    #[test]
    fn bfs_iterator_on_an_empty_graph_should_be_empty() {
        let g = DirectedGraph::new();
        let mut it = bfs_iter(&g);
        assert![it.next().is_none(), "Iterator should be empty"]
    }

    #[test]
    fn bfs_iterator_on_a_one_node_graph_should_return_one_node() {
        let mut g = DirectedGraph::new();
        g.add_vertex(VertexId(1));
        let mut it = bfs_iter(&g);
        assert_eq![
            it.next(),
            Some(VertexId(1)),
            "Iterator should return the only node"
        ];
        assert![it.next().is_none(), "Iterator should now be empty"]
    }

    #[test]
    fn bfs_iterator_from_on_a_one_node_graph_should_return_the_only_node() {
        let mut g = DirectedGraph::new();
        g.add_vertex(VertexId(1));
        let mut it = bfs_iter_from(&g, VertexId(1));
        assert_eq![
            it.next(),
            Some(VertexId(1)),
            "Iterator should return the only node"
        ];
        assert![it.next().is_none(), "Iterator should now be empty"]
    }

    #[test]
    fn bfs_iterator_return_reachable_nodes_in_a_breadth_first_search_order() {
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

        // BFS order from vertex 1
        let it = bfs_iter_from(&g, VertexId(1));
        assert_eq![
            it.collect::<Vec<VertexId>>(),
            vec!(
                VertexId(1),
                VertexId(2),
                VertexId(4),
                VertexId(5),
                VertexId(3),
                VertexId(6),
                VertexId(7)
            ),
            "BFS order is wrong when starting from Vertex 1"
        ];
    }

    #[test]
    fn bfs_iterator_does_not_loop_when_encountering_a_cycle() {
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

        let it = bfs_iter_from(&g, VertexId(1));
        assert_eq![
            it.collect::<Vec<VertexId>>().len(),
            5,
            "BFS returned an invalid length"
        ];
    }

    // BFS path

    #[test]
    fn bfs_path_iterator_return_path_for_all_reachable_nodes_in_a_breadth_first_search_order() {
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

        let it = bfs_path_iter_from(&g, VertexId(1));
        assert_eq![
            it.collect::<Vec<Path>>(),
            vec!(
                path_of(vec![1]),
                path_of(vec![1, 2]),
                path_of(vec![1, 4]),
                path_of(vec![1, 5]),
                path_of(vec![1, 2, 3]),
                path_of(vec![1, 2, 5]),
                path_of(vec![1, 4, 5]),
                path_of(vec![1, 4, 6]),
                path_of(vec![1, 4, 6, 7]),
                path_of(vec![1, 4, 6, 7, 2]),
                path_of(vec![1, 4, 6, 7, 2, 3]),
                path_of(vec![1, 4, 6, 7, 2, 5]),
            ),
            "BFS order is wrong when starting from Vertex 1"
        ];
    }
}
