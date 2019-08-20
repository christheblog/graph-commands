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

    pub fn remove_vertex(&mut self, _vertex_id: VertexId) -> bool {
        unimplemented!()
    }

    pub fn add_edge(&mut self, edge: Edge) {
        let Edge(v1, v2) = edge;
        self.add_vertex(v1);
        self.add_vertex(v2);
        self.edge_map.get_mut(&v1).unwrap().push(edge);
        if edge.0 != edge.1 {
            self.edge_map.get_mut(&v2).unwrap().push(edge);
        }
    }

    pub fn remove_edge(&mut self, _edge: Edge) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::DirectedGraph;
    use crate::graph::{Edge, VertexId};

    #[test]
    fn test_ne() {
        let mut graph_1 = DirectedGraph::new();
        graph_1.add_vertex(VertexId(0));
        graph_1.add_vertex(VertexId(1));
        graph_1.add_vertex(VertexId(2));

        graph_1.add_edge(Edge(VertexId(0), VertexId(1)));
        graph_1.add_edge(Edge(VertexId(0), VertexId(2)));
        graph_1.add_edge(Edge(VertexId(0), VertexId(0)));

        let mut graph_2 = DirectedGraph::new();
        graph_2.add_vertex(VertexId(0));
        graph_2.add_vertex(VertexId(1));
        graph_2.add_vertex(VertexId(2));

        graph_2.add_edge(Edge(VertexId(0), VertexId(0)));
        graph_2.add_edge(Edge(VertexId(0), VertexId(0)));
        graph_2.add_edge(Edge(VertexId(0), VertexId(1)));
        graph_2.add_edge(Edge(VertexId(0), VertexId(2)));
        graph_2.add_edge(Edge(VertexId(0), VertexId(2)));
        graph_2.add_edge(Edge(VertexId(0), VertexId(1)));

        assert_ne!(graph_1, graph_2);
    }
}
