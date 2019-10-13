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

/// Returns a new best first search iterator on the given graph
pub fn best_iter<F>(graph: &DirectedGraph, scorefn: F) -> BestFirstIter<F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    match graph.head_option() {
        None => empty_best_iter(graph, scorefn),
        Some(head) => bfs_iter_from(graph, scorefn, *head),
    }
}

/// Returns a new best first search iterator on the given graph, starting from the given start_vertex
pub fn bfs_iter_from<F>(
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
