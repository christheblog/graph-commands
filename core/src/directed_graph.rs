use crate::graph::Edge;
use std::collections::HashMap;

use crate::graph::*;

/// A directed graph structure that doesn't contain any information concerning the vertex or the
/// edge attributes
#[derive(Debug, PartialEq)]
pub struct DirectedGraph {
    // Each edge is indexed by both its vertices => 1 edge appears twice in the map
    edge_map: HashMap<VertexId, Vec<Edge>>,
}

impl DirectedGraph {
    pub fn new() -> DirectedGraph {
        DirectedGraph {
            edge_map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_count() == 0
    }

    pub fn head_option(&self) -> Option<&VertexId> {
        self.edge_map.keys().next()
    }

    pub fn vertex_count(&self) -> usize {
        self.edge_map.len()
    }

    pub fn edge_count(&self) -> usize {
        let mut total_count: usize = 0;
        for (_, edges) in &self.edge_map {
            total_count += edges.len();
        }
        // Each edge is saved twice => the count should be a multiple of 2, and divided by 2
        total_count / 2
    }

    pub fn contains_vertex(&self, vertex_id: VertexId) -> bool {
        self.edge_map.contains_key(&vertex_id)
    }

    pub fn vertices(&self) -> impl Iterator<Item = &VertexId> {
        self.edge_map.keys()
    }

    pub fn contains_edge(&self, edge: Edge) -> bool {
        let Edge(v1, v2) = edge;
        if self.contains_vertex(v1) && self.contains_vertex(v2) {
            // We need to look-up only for one of the vertices
            self.edge_map
                .get(&v1)
                .unwrap()
                .iter()
                .position(|x| *x == edge)
                .is_some()
        } else {
            false
        }
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edge_map
            .iter()
            .map(|(vertex_id, edges)| {
                edges
                    .iter()
                    .filter(move |Edge(from, _)| *vertex_id == *from)
            })
            .flatten()
    }

    pub fn outbound_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = &Edge> {
        self.edge_map
            .get(&vertex_id)
            .into_iter()
            .flat_map(|edges| edges.iter())
            .filter(move |e| e.0 == vertex_id)
    }

    pub fn inbound_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = &Edge> {
        self.edge_map
            .get(&vertex_id)
            .into_iter()
            .flat_map(|edges| edges.iter())
            .filter(move |e| e.1 == vertex_id)
    }

    pub fn degree_out(&self, vertex_id: VertexId) -> usize {
        self.outbound_edges(vertex_id).count()
    }

    pub fn degree_in(&self, vertex_id: VertexId) -> usize {
        self.inbound_edges(vertex_id).count()
    }

    pub fn add_vertex(&mut self, vertex_id: VertexId) -> bool {
        let mut contains_vertex = true;
        self.edge_map.entry(vertex_id).or_insert_with(|| {
            contains_vertex = false;
            vec![]
        });
        contains_vertex
    }

    pub fn remove_vertex(&mut self, vertex_id: VertexId) -> bool {
        let (found, edges) = match self.edge_map.get(&vertex_id) {
            None => (false, vec![]),
            Some(edges) => (true, edges.clone()),
        };
        // Removing all edges containing the vertex
        self.edge_map.remove(&vertex_id);
        edges.iter().for_each(|e| {
            self.remove_edge(*e);
        });
        found
    }

    pub fn add_edge(&mut self, edge: Edge) -> bool {
        if !self.contains_edge(edge) {
            let Edge(v1, v2) = edge;
            self.add_vertex(v1);
            self.add_vertex(v2);
            self.edge_map.get_mut(&v1).unwrap().push(edge);
            if edge.0 != edge.1 {
                self.edge_map.get_mut(&v2).unwrap().push(edge);
            }
            true
        } else {
            false
        }
    }

    pub fn remove_edge(&mut self, edge: Edge) -> bool {
        let Edge(src, dst) = edge;
        self.edge_map
            .get_mut(&src)
            .map(|edges| edges.retain(|e| *e != edge));
        self.edge_map
            .get_mut(&dst)
            .map(|edges| edges.retain(|e| *e != edge));
        true // FIXME how to quickly check to know if we have removed something or not ?
    }
}

#[cfg(test)]
mod test {
    use super::DirectedGraph;
    use crate::graph::{Edge, VertexId};

    #[test]
    fn test_add_vertex() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(0));
        assert!(digraph.contains_vertex(vertex(0)));
    }

    #[test]
    fn test_remove_existing_vertex_work() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(0));
        assert!(digraph.contains_vertex(vertex(0)));
        assert!(digraph.remove_vertex(vertex(0)));
        assert!(!digraph.contains_vertex(vertex(0)));
    }

    #[test]
    fn test_remove_existing_vertex_removes_all_related_edges_too() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(0));
        digraph.add_vertex(vertex(1));
        digraph.add_vertex(vertex(2));
        digraph.add_vertex(vertex(3));
        digraph.add_edge(edge(1, 2));
        digraph.add_edge(edge(3, 1));
        digraph.add_edge(edge(2, 3));
        // Removing vertex 1 should remove both edges containing it
        assert!(digraph.remove_vertex(vertex(1)));
        assert!(!digraph.contains_edge(edge(1, 2)));
        assert!(!digraph.contains_edge(edge(3, 1)));
        // Still contain unrelated edge
        assert!(digraph.contains_edge(edge(2, 3)));
    }

    #[test]
    fn test_remove_non_existing_vertex_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(0));
        assert!(digraph.contains_vertex(vertex(0)));
        assert!(!digraph.remove_vertex(vertex(1)));
        assert!(digraph.contains_vertex(vertex(0)));
    }

    #[test]
    fn test_add_edge_with_existing_vertices_works() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(0));
        digraph.add_vertex(vertex(1));
        // No edges yet
        assert!(!digraph.contains_edge(edge(0, 1)));
        assert!(!digraph.contains_edge(edge(1, 0)));
        // Adding edge
        digraph.add_edge(edge(1, 0));
        assert!(!digraph.contains_edge(edge(0, 1)));
        assert!(digraph.contains_edge(edge(1, 0)));
    }

    #[test]
    fn test_add_edge_with_non_existing_vertices_works() {
        let mut digraph = DirectedGraph::new();
        assert!(!digraph.contains_vertex(vertex(0)));
        assert!(!digraph.contains_vertex(vertex(1)));
        // Adding edge
        digraph.add_edge(edge(1, 0));
        assert!(digraph.contains_edge(edge(1, 0)));
    }

    #[test]
    fn test_adding_an_edge_does_not_add_the_reverted_edge() {
        let mut digraph = DirectedGraph::new();
        digraph.add_edge(edge(1, 0));
        assert!(!digraph.contains_edge(edge(0, 1)));
    }

    #[test]
    fn test_removing_an_edge_removes_only_that_edge() {
        let mut digraph = DirectedGraph::new();
        digraph.add_edge(edge(1, 0));
        digraph.add_edge(edge(2, 1));
        digraph.add_edge(edge(0, 1));
        // Remove edge 0->1
        digraph.remove_edge(edge(0, 1));
        assert!(!digraph.contains_edge(edge(0, 1)));
        // Unremoved edges are still present
        assert!(digraph.contains_edge(edge(1, 0)));
        assert!(digraph.contains_edge(edge(2, 1)));
    }

    #[test]
    fn test_removing_an_edge_removes_all_occurences_of_this_edge() {
        let mut digraph = DirectedGraph::new();
        // 3 times same edge is added
        digraph.add_edge(edge(0, 1));
        digraph.add_edge(edge(0, 1));
        digraph.add_edge(edge(0, 1));
        // others
        digraph.add_edge(edge(1, 2));
        digraph.add_edge(edge(1, 3));

        assert!(digraph.edge_count() == 3);
        digraph.remove_edge(edge(0, 1));
        // All occurences of edge(0,0) have been removed
        assert!(digraph.edge_count() == 2);
        assert!(!digraph.contains_edge(edge(0, 1)));
    }

    // Testing Idem-Potent behavior

    #[test]
    fn test_removing_non_existing_vertex_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(100));
        digraph.add_vertex(vertex(200));
        digraph.add_edge(edge(1, 0));
        digraph.add_edge(edge(2, 1));
        digraph.add_edge(edge(0, 1));
        assert!(digraph.vertex_count() == 5);
        assert!(digraph.edge_count() == 3);
        // Remove non-existing vertex 999
        digraph.remove_vertex(vertex(999));
        assert!(digraph.vertex_count() == 5);
        assert!(digraph.edge_count() == 3);
    }

    #[test]
    fn test_remove_non_existing_edge_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        digraph.add_edge(edge(1, 0));
        digraph.add_edge(edge(2, 1));
        digraph.add_edge(edge(0, 1));
        assert!(digraph.edge_count() == 3);
        // Remove non-existing edge 7->8
        digraph.remove_edge(edge(7, 8));
        assert!(digraph.vertex_count() == 3);
        assert!(digraph.edge_count() == 3);
    }

    #[test]
    fn test_adding_a_vertex_several_times_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(100));
        digraph.add_edge(edge(0, 1));
        digraph.add_edge(edge(1, 2));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 2);
        // Readding a vertex
        digraph.add_vertex(vertex(100));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 2);
    }

    #[test]
    fn test_removing_a_vertex_several_times_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        digraph.add_vertex(vertex(100));
        digraph.add_vertex(vertex(200));
        digraph.add_edge(edge(0, 1));
        digraph.add_edge(edge(1, 2));
        assert!(digraph.vertex_count() == 5);
        assert!(digraph.edge_count() == 2);
        // Removing a vertex
        digraph.remove_vertex(vertex(100));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 2);
        // Removing again
        digraph.remove_vertex(vertex(100));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 2);
    }

    #[test]
    fn test_adding_an_edge_several_times_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        digraph.add_edge(edge(0, 1));
        digraph.add_edge(edge(1, 2));
        digraph.add_edge(edge(1, 3));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 3);
        // Addin an edge again
        digraph.add_edge(edge(0, 1));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 3);
    }

    #[test]
    fn test_removing_an_edge_several_times_has_no_effect() {
        let mut digraph = DirectedGraph::new();
        // 3 times same edge is added
        digraph.add_edge(edge(0, 1));
        digraph.add_edge(edge(1, 2));
        digraph.add_edge(edge(1, 3));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 3);
        // Removing an edge
        digraph.remove_edge(edge(1, 3));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 2);
        // Removing the edge again
        digraph.remove_edge(edge(1, 3));
        assert!(digraph.vertex_count() == 4);
        assert!(digraph.edge_count() == 2);
        assert!(!digraph.contains_edge(edge(1, 3)));
    }

    // Helpers

    fn vertex(id: u64) -> VertexId {
        VertexId(id)
    }

    fn edge(src: u64, dst: u64) -> Edge {
        Edge(VertexId(src), VertexId(dst))
    }
}
