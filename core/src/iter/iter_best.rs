///! Graph Iterator implementation
use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::iter::iter_datastructure::{PriorityQueue, SearchQueue};
use crate::path::Path;

use crate::path::ScoredPath;
use std::collections::HashSet;

/// Best-First search iterator

pub struct BestFirstIter<'a, F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    queue: PriorityQueue<ScoredPath>,
    visited: HashSet<VertexId>,
    graph: &'a DirectedGraph,
    scorefn: F,
}

impl<'a, F> Iterator for BestFirstIter<'a, F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    type Item = ScoredPath;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop() {
            None => None,
            Some(weighted_path) => {
                let vid = weighted_path.path.last().unwrap();
                self.graph
                    .outbound_edges(*vid)
                    .map(|Edge(_, v)| v)
                    .for_each(|v| {
                        if !self.visited.contains(v) {
                            self.visited.insert(*v);
                            let new_path = weighted_path.path.append(*v);
                            let new_scored_path = ScoredPath {
                                path: weighted_path.path.append(*v),
                                score: (self.scorefn)(self.graph, &new_path),
                            };
                            self.queue.push(new_scored_path)
                        }
                    });
                Some(weighted_path)
            }
        }
    }
}

/// Returns a new best first search iterator on the given graph, starting from the given start_vertex
pub fn best_iter_from<F>(
    graph: &DirectedGraph,
    scorefn: F,
    start_vertex: VertexId,
) -> BestFirstIter<F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    let mut iter = empty_best_iter(graph, scorefn);
    iter.queue.push(ScoredPath {
        path: Path::empty().append(start_vertex),
        score: 1,
    });
    iter.visited.insert(start_vertex);
    iter
}

/// Builds an empty iterator from a given graph.
fn empty_best_iter<F>(graph: &DirectedGraph, scorefn: F) -> BestFirstIter<F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    BestFirstIter {
        queue: PriorityQueue::<ScoredPath>::new(),
        visited: HashSet::new(),
        graph: graph,
        scorefn: scorefn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Iterator;

    // score returns the node id of the last node of the path
    fn score(_graph: &DirectedGraph, path: &Path) -> i64 {
        match path.last() {
            Some(VertexId(x)) => *x as i64,
            None => 0,
        }
    }

    #[test]
    fn best_iterator_from_on_a_one_node_graph_should_return_a_one_node_path() {
        let mut g = DirectedGraph::new();
        g.add_vertex(VertexId(1));
        let mut it = best_iter_from(&g, score, VertexId(1));
        assert_eq![
            it.next(),
            Some(ScoredPath {
                path: Path {
                    vertices: vec![VertexId(1)]
                },
                score: 1
            }),
            "Iterator should return the only one-node path"
        ];
        assert![it.next().is_none(), "Iterator should now be empty"]
    }

    #[test]
    fn best_iterator_return_reachable_nodes_in_a_breadth_first_search_order() {
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
        let it = best_iter_from(&g, score, VertexId(1));
        assert_eq![
            it.collect::<Vec<ScoredPath>>(),
            vec![
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1)]
                    },
                    score: 1
                },
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1), VertexId(5)]
                    },
                    score: 5
                },
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1), VertexId(4)]
                    },
                    score: 4
                },
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1), VertexId(4), VertexId(6)]
                    },
                    score: 6
                },
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1), VertexId(4), VertexId(6), VertexId(7)]
                    },
                    score: 7
                },
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1), VertexId(2)]
                    },
                    score: 2
                },
                ScoredPath {
                    path: Path {
                        vertices: vec![VertexId(1), VertexId(2), VertexId(3)]
                    },
                    score: 3
                }
            ],
            "Best order is wrong when starting from Vertex 1"
        ];
    }

    #[test]
    fn best_iterator_does_not_loop_when_encountering_a_cycle() {
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

        let it = best_iter_from(&g, score, VertexId(1));
        assert_eq![
            it.collect::<Vec<ScoredPath>>().len(),
            5,
            "Best returned an invalid length"
        ];
    }
}
