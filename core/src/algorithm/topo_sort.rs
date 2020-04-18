use crate::directed_graph::DirectedGraph;
use crate::graph::Edge;
use crate::graph::VertexId;
use std::collections::HashSet;
use std::collections::LinkedList;

/// Tests if te graph is a Directed Acyclic Graph (DAG)
pub fn is_dag(graph: &DirectedGraph) -> bool {
    topological_sort(graph).is_some()
}

/// Computes a topological order for a Graph
/// See Kahn's algorithm: https://en.wikipedia.org/wiki/Topological_sorting
pub fn topological_sort(graph: &DirectedGraph) -> Option<Vec<VertexId>> {
    let mut res: Vec<VertexId> = vec![];
    let mut start_vertices: LinkedList<VertexId> = find_start_vertices(graph).collect();
    // Keeping track of removed edges  (ie visited, since we don't want to remove them from the graph)
    let mut removed_edges = HashSet::<Edge>::new();

    while let Some(start) = start_vertices.pop_front() {
        res.push(start);
        let mut children: Vec<Edge> = graph.outbound_edges(start).map(|x| *x).collect();
        children.retain(|x| !removed_edges.contains(x));
        for edge in children {
            removed_edges.insert(edge);
            let Edge(_, dest) = edge;
            // No other incoming edges (ie no other edges, or already visited edges)
            if graph.inbound_edges(dest).all(|e| removed_edges.contains(e)) {
                start_vertices.push_back(dest);
            }
        }
    }

    if removed_edges.len() != graph.edge_count() {
        None
    } else {
        Some(res)
    }
}

// Finds a start a node with no inbound edges
fn find_start_vertices(graph: &DirectedGraph) -> impl Iterator<Item = VertexId> + '_ {
    graph
        .vertices()
        .map(|x| *x)
        .filter(move |vid| is_start_vertex(graph, vid))
}

// A start vertex is a vertex that doesn't have any incoming edge
fn is_start_vertex(graph: &DirectedGraph, vid: &VertexId) -> bool {
    graph.inbound_edges(*vid).count() == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hash;

    #[test]
    fn topological_order_on_an_empty_graph_is_empty() {
        let g = DirectedGraph::new();
        assert_eq!(topological_sort(&g), Some(vec![]));
    }

    #[test]
    fn topological_order_on_a_dag_returns_correct_order_1() {
        // DAG taken from https://en.wikipedia.org/wiki/Topological_sorting
        let mut g = DirectedGraph::new();
        g.add_edge(edge(5, 11));
        g.add_edge(edge(11, 2));
        g.add_edge(edge(7, 11));
        g.add_edge(edge(11, 9));
        g.add_edge(edge(11, 10));
        g.add_edge(edge(7, 8));
        g.add_edge(edge(8, 9));
        g.add_edge(edge(3, 8));
        g.add_edge(edge(3, 10));
        let sorted = topological_sort(&g).unwrap();
        // First level should be starting nodes. order doesn't matter
        assert_eq!(
            set(&sorted[0..3]),
            set(&[VertexId(7), VertexId(3), VertexId(5)])
        );
        // Second Level
        assert_eq!(set(&sorted[3..5]), set(&[VertexId(11), VertexId(8)]));
        // Third level
        assert_eq!(
            set(&sorted[5..]),
            set(&[VertexId(2), VertexId(9), VertexId(10)])
        );
    }

    #[test]
    fn topological_order_on_a_dag_returns_correct_order_2() {
        // DAG with diamond pattern: 2->5->7, 2->4->6->7
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(2, 4));
        g.add_edge(edge(4, 6));
        g.add_edge(edge(2, 5));
        g.add_edge(edge(5, 7));
        g.add_edge(edge(6, 7));
        g.add_edge(edge(7, 8));
        let sorted = topological_sort(&g).unwrap();
        // First level should be starting nodes. order doesn't matter
        assert_eq!(set(&sorted[0..2]), set(&[VertexId(1), VertexId(3)]));
        // Second Level
        assert_eq!(set(&sorted[2..4]), set(&[VertexId(2), VertexId(4)]));
        // Third level
        assert_eq!(set(&sorted[4..6]), set(&[VertexId(5), VertexId(6)]));
        // Fourth level
        assert_eq!(sorted[6], VertexId(7));
        // Fifth level
        assert_eq!(sorted[7], VertexId(8));
    }

    #[test]
    fn topological_sort_should_work_on_a_big_number_of_chained_vertices() {
        let mut g = DirectedGraph::new();
        let n = 25000;
        for i in 1..n {
            g.add_edge(edge(i, i + 1));
        }
        // One of the possible topological order is :
        assert_eq!(
            topological_sort(&g),
            Some((1..n + 1).map(|id| VertexId(id)).collect())
        );
    }

    #[test]
    fn topological_order_on_a_graph_with_a_cycle_is_empty_1() {
        let mut g = DirectedGraph::new();
        let n = 25000;
        for i in 1..n {
            g.add_edge(edge(i, i + 1));
        }
        g.add_edge(edge(25000, 1)); // creating a cycle here
        assert!(topological_sort(&g).is_none());
    }

    #[test]
    fn topological_order_on_a_graph_with_a_cycle_is_empty_2() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(5, 11));
        g.add_edge(edge(11, 2));
        g.add_edge(edge(7, 11));
        g.add_edge(edge(11, 9));
        g.add_edge(edge(11, 10));
        g.add_edge(edge(7, 8));
        g.add_edge(edge(8, 9));
        g.add_edge(edge(3, 8));
        g.add_edge(edge(3, 10));
        g.add_edge(edge(10, 5)); // cycle here : 5->11->10->5
        assert!(topological_sort(&g).is_none());
    }

    #[test]
    fn topological_order_on_a_graph_with_a_cycle_is_empty_3() {
        let mut g = DirectedGraph::new();
        // contains 2 cycles :
        // 3->4->5->6->3, 1->3->4->5->6->1
        g.add_edge(edge(1, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 7));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 7));
        g.add_edge(edge(5, 6));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(6, 1));
        g.add_edge(edge(6, 3));
        assert!(topological_sort(&g).is_none());
    }

    // Helpers

    fn edge(src: u64, dst: u64) -> Edge {
        Edge(VertexId(src), VertexId(dst))
    }

    fn set<T: Eq + Hash + Clone>(v: &[T]) -> HashSet<T> {
        v.iter().map(|x| x.clone()).collect::<HashSet<T>>()
    }
}
