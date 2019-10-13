use crate::graph::{Edge, VertexId};
///! Graph path implementation
use core::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Path {
    // FIXME used immutable Linked list here for cheap append + structural sharing
    vertices: Vec<VertexId>,
}

impl Path {
    pub fn empty() -> Path {
        Path { vertices: vec![] }
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn contains(&self, vertex: &VertexId) -> bool {
        self.vertices.contains(vertex)
    }

    pub fn to_edge_list(&self) -> impl Iterator<Item = Edge> + '_ {
        self.vertices
            .windows(2)
            .map(|slice| Edge(slice[0], slice[1]))
    }

    pub fn last(&self) -> Option<&VertexId> {
        self.vertices.last()
    }

    /// Append a vertex to a path
    pub fn append(&self, vertex: VertexId) -> Path {
        // FIXME use a data structure with structural sharing to avoid the clone
        let mut new_path = Path {
            vertices: self.vertices.clone(),
        };
        new_path.vertices.push(vertex);
        new_path
    }
}

/// Scored path - understand as a weighted path

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ScoredPath {
    pub path: Path,
    pub score: i64,
}

impl PartialOrd for ScoredPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}
