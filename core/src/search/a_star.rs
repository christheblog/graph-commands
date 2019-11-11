use crate::directed_graph::DirectedGraph;
use crate::graph::VertexId;
use crate::iter;
use crate::iter::constraint::Constraint;
use crate::path::Path;
use crate::path::ScoredPath;

pub fn shortest_path<H>(
    graph: &DirectedGraph,
    heuristicfn: H,
    start: VertexId,
    end: VertexId,
) -> Option<ScoredPath>
where
    H: Fn(&DirectedGraph, &Path) -> i64,
{
    iter::iter_best::best_iter_from(graph, heuristicfn, start)
        .find(|sp| sp.path.last().map(|x| *x) == Some(end))
}

pub fn constrained_shortest_path<H>(
    graph: &DirectedGraph,
    heuristicfn: H,
    start: VertexId,
    end: VertexId,
    constraints: Vec<Constraint>,
) -> Option<ScoredPath>
where
    H: Fn(&DirectedGraph, &Path) -> i64,
{
    iter::iter_best_constraint::constrained_best_iter_from(graph, heuristicfn, constraints, start)
        .find(|sp| sp.path.last().map(|x| *x) == Some(end))
}

#[cfg(test)]
mod tests {
    use super::*;

    // score returns the node id of the last node of the path
    fn heuristic(_graph: &DirectedGraph, _path: &Path) -> i64 {
        unimplemented!()
    }

    fn build_test_graph() -> DirectedGraph {
        unimplemented!()
    }

    // Shortest path

    #[test]
    fn shortest_path_should_find_the_shortest_path_when_it_exists() {}

    #[test]
    fn shortest_path_should_return_none_when_no_path_exists() {}

    // Shortest path with constraint

    #[test]
    fn shortest_path_should_find_the_shortest_path_satisfying_constraints_when_it_exists() {}

    #[test]
    fn shortest_path_should_return_none_when_no_shortest_path_satisfying_constraints_exists() {}

}
