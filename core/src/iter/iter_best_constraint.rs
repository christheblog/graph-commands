use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
///! Best-First search Iterator with constraint implementation
use crate::iter::constraint::Constraint;
use crate::iter::iter_datastructure::{MinPriorityQueue, SearchQueue};
use crate::path::Path;

use crate::path::ScoredPath;

/// Best-First search iterator

pub struct ConstrainedBestFirstIter<'a, F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    queue: MinPriorityQueue<ScoredPath>,
    graph: &'a DirectedGraph,
    scorefn: F,
    constraints: Vec<Constraint>,
}

impl<'a, F> Iterator for ConstrainedBestFirstIter<'a, F>
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
                        let new_path = weighted_path.path.append(*v);
                        let new_scored_path = ScoredPath {
                            path: weighted_path.path.append(*v),
                            score: (self.scorefn)(self.graph, &new_path),
                        };
                        // If the newly generated partial path is still a good candidate
                        // to satisfy all constraints, we put it in the queue
                        if check_all_partial_constraints(&new_scored_path, &self.constraints) {
                            self.queue.push(new_scored_path)
                        }
                    });
                Some(weighted_path)
            }
        }
    }
}

fn check_all_partial_constraints(sp: &ScoredPath, constraints: &Vec<Constraint>) -> bool {
    constraints.iter().all(|c| c.check_partial(sp))
}

/// Returns a new constrained best first search iterator on the given graph,
/// starting from the given start_vertex
pub fn constrained_best_iter_from<F>(
    graph: &DirectedGraph,
    scorefn: F,
    constraints: Vec<Constraint>,
    start_vertex: VertexId,
) -> ConstrainedBestFirstIter<F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    let path = Path::empty().append(start_vertex);
    let score = scorefn(graph, &path);
    let mut iter = empty_constrained_best_iter(graph, scorefn, constraints);
    iter.queue.push(ScoredPath { path, score });
    iter
}

/// Builds an empty constrained iterator from a given graph.
fn empty_constrained_best_iter<F>(
    graph: &DirectedGraph,
    scorefn: F,
    constraints: Vec<Constraint>,
) -> ConstrainedBestFirstIter<F>
where
    F: Fn(&DirectedGraph, &Path) -> i64,
{
    ConstrainedBestFirstIter {
        queue: MinPriorityQueue::<ScoredPath>::new(),
        graph: graph,
        scorefn: scorefn,
        constraints: constraints,
    }
}
