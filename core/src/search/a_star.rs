use crate::directed_graph::DirectedGraph;
use crate::graph::VertexId;
use crate::iter;
use crate::iter::constraint::Constraint;
use crate::path::Path;
use crate::path::ScoredPath;

/// Find the shortest path using A* algorithm
///
/// g: cost function of the path so far
/// h: heuristic
/// start
/// end
pub fn shortest_path<G, H>(
    graph: &DirectedGraph,
    g: G, // computing current cost of the path so far
    h: H, // heuristic
    start: VertexId,
    end: VertexId,
) -> Option<ScoredPath>
where
    G: Fn(&DirectedGraph, &Path) -> i64,
    H: Fn(&DirectedGraph, &Path) -> i64,
{
    iter::iter_best::best_iter_from(
        graph,
        |dg, path| g(dg, path) + h(dg, path), // f = g + h
        start,
    )
    .find(|sp| sp.path.last().map(|x| *x) == Some(end))
}

/// Find the shortest path using A* algorithm but taking into account
/// the list of provided constraints
///
/// g: cost function of the path so far
/// h: heuristic
/// start
/// end
pub fn constrained_shortest_path<G, H>(
    graph: &DirectedGraph,
    g: G, // computing current cost of the path so far
    h: H, // heuristic
    start: VertexId,
    end: VertexId,
    constraints: Vec<Constraint>,
) -> Option<ScoredPath>
where
    G: Fn(&DirectedGraph, &Path) -> i64,
    H: Fn(&DirectedGraph, &Path) -> i64,
{
    iter::iter_best_constraint::constrained_best_iter_from(
        graph,
        |dg, path| g(dg, path) + h(dg, path), // f = g + h
        constraints,
        start,
    )
    .find(|sp| sp.path.last().map(|x| *x) == Some(end))
}

/// Zero information heuristic function
/// Equivalent to not having an heuristic
pub fn zero_heuristic(_graph: &DirectedGraph, _path: &Path) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Edge;

    // score returns the node id of the last node of the path
    fn cost(_graph: &DirectedGraph, path: &Path) -> i64 {
        path.vertices.iter().map(|VertexId(x)| *x as i64).sum()
    }

    // Shortest path

    #[test]
    fn shortest_path_should_find_the_shortest_path_when_it_exists() {
        let g = build_test_graph();
        assert_eq![
            shortest_path(&g, &cost, &zero_heuristic, VertexId(1), VertexId(7)),
            Some(ScoredPath {
                path: Path {
                    vertices: vec![VertexId(1), VertexId(4), VertexId(6), VertexId(7)]
                },
                score: 18
            })
        ]
    }

    #[test]
    fn shortest_path_should_return_none_when_no_path_exists() {
        let g = build_test_graph();
        assert![shortest_path(&g, &cost, &zero_heuristic, VertexId(1), VertexId(8)).is_none()]
    }

    // Shortest path with constraint

    #[test]
    fn shortest_path_should_find_the_shortest_path_satisfying_constraints_when_it_exists() {}

    #[test]
    fn shortest_path_should_return_none_when_no_shortest_path_satisfying_constraints_exists() {}

    // Helpers

    fn build_test_graph() -> DirectedGraph {
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
        g
    }

    fn edge_from(src: u64, end: u64) -> Edge {
        Edge(VertexId(src), VertexId(end))
    }

}
