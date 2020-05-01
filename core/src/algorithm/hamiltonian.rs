//! Find Hamiltonian paths in a directed graph

use crate::directed_graph::DirectedGraph;
use crate::iter;
use crate::path::Path;

/// Checks if the graph is Hamiltonian (ie contains an hamiltonian path)
/// By convention an empty graph is hamiltonian
pub fn is_hamiltonian(graph: &DirectedGraph) -> bool {
    graph.is_empty() || first_path(graph).is_some()
}

/// Iterates on the hamiltonian paths of the graph if it exists
/// Naive algorithm implementation. Will do a DFS for each vertex of the graph,
pub fn iter_hamiltonian_paths(graph: &DirectedGraph) -> impl Iterator<Item = Path> + '_ {
    // FIXME check if path is connected first, else return empty iterator straght away
    let count = graph.vertex_count();
    graph
        .vertices()
        .flat_map(move |vid| iter::iter_depth::dfs_iter_path_from(graph, *vid))
        .filter(move |p| p.size() == count)
}

/// Finds an hamiltonian path of the graph if it exists
pub fn first_path(graph: &DirectedGraph) -> Option<Path> {
    iter_hamiltonian_paths(graph).next()
}

/// Checks if the path is an hamiltonian for the given graph
/// Assumption : the path is coming from the given graph (ie this is a valid path with respect to the graph)
pub fn is_path_hamiltonian(path: &Path, graph: &DirectedGraph) -> bool {
    !path.contains_cycle() && path.size() == graph.vertex_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Iterator;
    use crate::graph::Edge;
    use crate::graph::VertexId;

    #[test]
    fn is_hamiltonian_should_return_true_for_an_empty_graph_by_convention() {
        let g = DirectedGraph::new();
        assert![is_hamiltonian(&g), "An empty graph should be Hamiltonian"]
    }

    #[test]
    fn first_path_should_return_none_if_there_is_no_hamiltonian_path() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(2, 4));
        g.add_edge(edge(2, 5));
        g.add_edge(edge(4, 6));
        g.add_edge(edge(5, 7));
        g.add_edge(edge(6, 7));
        g.add_edge(edge(7, 8));

        assert![
            first_path(&g).is_none(),
            "The graph has no Hamiltonian path"
        ]
    }

    #[test]
    fn first_path_should_return_a_path_if_graph_has_one_hamiltonian_cycle() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 1));
        g.add_edge(edge(2, 4));

        assert![
            first_path(&g).is_some(),
            "The graph has no Hamiltonian path"
        ];

        assert![
            first_path(&g).filter(|p| p.size()==5).is_some(),
            "The length of a Hamiltonian path should be the number of vertices of the graph"
        ];
    }

    #[test]
    fn iter_hamiltonian_paths_should_return_all_distinct_hamiltonian_path_1() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 5));

        let hamiltonian_paths: Vec<Path> = iter_hamiltonian_paths(&g).collect();

        assert_eq![
            hamiltonian_paths.len(),
            1,
            "The graph should have 1 hamiltonian path."
        ];
    }

    #[test]
    fn iter_hamiltonian_paths_should_return_all_distinct_hamiltonian_path_2() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 1));

        let hamiltonian_paths: Vec<Path> = iter_hamiltonian_paths(&g).collect();

        assert_eq![
            hamiltonian_paths.len(),
            5,
            "The graph should have 5 hamiltonian path."
        ];
    }

    #[test]
    fn iter_hamiltonian_paths_should_return_all_distinct_hamiltonian_path_3() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 1));
        g.add_edge(edge(2, 4));
        g.add_edge(edge(5, 3));
        g.add_edge(edge(3, 1));

        let hamiltonian_paths: Vec<Path> = iter_hamiltonian_paths(&g).collect();

        assert_eq![
            hamiltonian_paths.len(),
            10,
            "The graph should have 10 hamiltonian path."
        ];
    }

    #[test]
    fn iter_hamiltonian_paths_should_return_all_distinct_hamiltonian_path_4() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 1));
        g.add_edge(edge(2, 4));
        g.add_edge(edge(5, 3));
        g.add_edge(edge(3, 1));

        let hamiltonian_paths: Vec<Path> = iter_hamiltonian_paths(&g).collect();

        assert_eq![
            hamiltonian_paths.len(),
            10,
            "The graph should have 10 hamiltonian path."
        ];
    }

    // Helpers

    fn edge(src: u64, dst: u64) -> Edge {
        Edge(VertexId(src), VertexId(dst))
    }
}
