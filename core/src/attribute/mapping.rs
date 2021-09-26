//! Provides a mapping of attributes for vertices and edges
//! Since Vertices are represented by ids, this is the way to add information to a graph
//! Note: This keeps the graph structure and information attached to it separated
use crate::graph::*;
use std::collections::HashMap;
use std::hash::Hash;

pub type VertexAttrMapping<V> = AttributeMapping<VertexId, V>;
pub type EdgeAttrMapping<V> = AttributeMapping<Edge, V>;


/// Empty Vertex Mapping
pub fn no_vertex_mapping<V>() -> VertexAttrMapping<V> {
    AttributeMapping::<VertexId, V>::new("empty")
}

/// Empty Edge Mapping
pub fn no_edge_mapping<V>() -> EdgeAttrMapping<V> {
    AttributeMapping::<Edge, V>::new("empty")
}


/// Mapping between an edge and an attribute value
/// There should be one mapping per attribute
pub struct AttributeMapping<K: Eq + Hash, V> {
    name: String,
    mapping: HashMap<K, V>,
}

impl<K: Eq + Hash, V> AttributeMapping<K, V> {
    pub fn new<Key: Eq + Hash, Value>(name: &str) -> AttributeMapping<Key, Value> {
        AttributeMapping {
            name: name.to_string(),
            mapping: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add(&mut self, e: K, value: V) -> bool {
        self.mapping.insert(e, value);
        true
    }

    pub fn remove(&mut self, e: &K) -> bool {
        self.mapping.remove(e).is_some()
    }

    // Representing this mapping as closure

    pub fn as_closure<'a>(&'a self) -> impl Fn(&K) -> Option<&'a V> {
        move |e: &K| self.mapping.get(e)
    }

    pub fn as_closure_with_defaults<'a>(&'a self, default_value: &'a V) -> impl Fn(&K) -> &'a V {
        move |e: &K| self.mapping.get(e).unwrap_or(default_value)
    }
}
