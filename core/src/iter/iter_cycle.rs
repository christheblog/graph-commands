use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::iter::iter_datastructure::{SearchQueue, Stack};
use crate::path::Path;

use itertools::Itertools;
use std::collections::HashSet;

/// Represents a cycle.
/// In a cycle representation no element appears twice.
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct Cycle {
    vertices: Vec<VertexId>,
}

impl Cycle {
    pub fn from_path(path: &Path) -> Option<Cycle> {
        Cycle::from_vertices(&path.vertices)
    }

    /// A cycle object can be built only from more than 2 vertices, with no duplication of vertex
    pub fn from_vertices(vertices: &Vec<VertexId>) -> Option<Cycle> {
        if vertices.len() < 2 || vertices.len() != vertices.iter().unique().count() {
            None
        } else {
            Some(Cycle {
                vertices: vertices.to_vec(),
            })
        }
    }

    /// Converts a cycle into a path
    /// Path will include first and last element that are identical
    pub fn as_path(&self) -> Path {
        Path::from(&self.vertices)
    }

    /// length of the cycle
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Provide the cycle as an iterator of VertexId
    pub fn iter(&self) -> impl Iterator<Item = &VertexId> {
        self.vertices.iter()
    }

    /// Returns a canonical representation of this cycle
    /// This is to help comparing cycles as they can be detected with any permutation of vertices
    pub fn canonical(&self) -> Cycle {
        // Canonical repr is will just rotate the to have the smallest vertex first
        let (min_position, _) = self
            .vertices
            .iter()
            .enumerate()
            .min_by(|x, y| x.1.cmp(y.1))
            .expect("It shouldn't have been possible to build a cycle with no element");
        let mut min_first = self.vertices.clone();
        min_first.rotate_left(min_position);
        Cycle {
            vertices: min_first,
        }
    }

    /// Returns true if this cycle is already in canonical representation
    pub fn is_canonical(&self) -> bool {
        let min_vid = self
            .vertices
            .iter()
            .min()
            .expect("It shouldn't have been possible to build a cycle with no element");
        self.vertices.first() == Some(min_vid)
    }
}

pub struct CycleIter<'a> {
    stack: Stack<Path>,
    returned: HashSet<Cycle>,
    graph: &'a DirectedGraph,
}

/// Iterates over all the unique cycles from a Graph
impl<'a> Iterator for CycleIter<'a> {
    type Item = Cycle;

    fn next(&mut self) -> Option<Cycle> {
        // DFS until a path contains contains a cycle
        while let Some(path) = self.stack.pop() {
            match extract_canonical_cycle_from_last(&path) {
                Some(cycle) if !self.returned.contains(&cycle) => {
                    self.returned.insert(cycle.clone());
                    return Some(cycle);
                }
                // Cycle has already been pushed into the iterator
                Some(_) => (),
                None => {
                    let last = path.last().unwrap();
                    self.graph
                        .outbound_edges(*last)
                        .map(|Edge(_, v)| v)
                        .for_each(|v| self.stack.push(path.append(*v)));
                }
            }
        }
        // If we reach this stage, stack is empty,
        // No more cycle to be found, so ending iteration
        None
    }
}

/// Returns a new cycle iterator on the given graph
pub fn cycle_iter(graph: &DirectedGraph) -> CycleIter {
    let starting_vertices = find_starting_edges(graph);
    let mut cycle_iter = empty_cycle_iter(graph);
    for vertex in starting_vertices {
        let path = Path::from(&vec![*vertex]);
        cycle_iter.stack.push(path);
    }
    cycle_iter
}

/// Builds an empty iterator from a given graph.
fn empty_cycle_iter(graph: &DirectedGraph) -> CycleIter {
    CycleIter {
        stack: Stack::<Path>::new(),
        returned: HashSet::new(),
        graph: graph,
    }
}

// Note: This is assuming a connected graph
fn find_starting_edges(graph: &DirectedGraph) -> Vec<&VertexId> {
    let mut res = graph
        .vertices()
        .filter(|vid| graph.inbound_edges(**vid).count() == 0)
        .collect::<Vec<&VertexId>>();

    // If no vertex with no inbound edges can be found, we need to randomly add a vertex
    if res.is_empty() {
        graph.head_option().iter().for_each(|x| res.push(x));
    }
    res
}

// Helpers

// Extracts a cycle made by the last element of a Path
// Example: Path { vertices: [7,5,3,2,5,8,9,2]} should return Some(Cycle { vertices: [2,5,8,9] })
fn extract_canonical_cycle_from_last(path: &Path) -> Option<Cycle> {
    path.last()
        .and_then(|last| {
            path.vertices[..path.vertices.len() - 1]
                .iter()
                .rposition(|x| x == last)
        })
        .map(|start| Cycle {
            vertices: path.vertices[start..path.vertices.len() - 1].to_vec(),
        })
        .map(|c| c.canonical())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_cycle_should_always_start_with_smallest_vertex() {
        assert_eq!(
            cano_cycle(vec![7, 5, 1, 2, 4]).unwrap(),
            cycle(vec![1, 2, 4, 7, 5]).unwrap()
        );
        assert_eq!(
            cano_cycle(vec![7, 5, 22, 2, 4]).unwrap(),
            cycle(vec![2, 4, 7, 5, 22]).unwrap()
        );
        assert_eq!(
            cano_cycle(vec![1, 5, 7, 2, 4]).unwrap(),
            cycle(vec![1, 5, 7, 2, 4]).unwrap()
        );
    }

    #[test]
    fn extract_canonical_cycle_from_last_should_return_none_if_there_is_no_cycle_at_all() {
        assert!(extract_canonical_cycle_from_last(&path(vec![7, 3, 2, 5, 8, 9, 12])).is_none());
    }

    #[test]
    fn extract_canonical_cycle_from_last_should_return_none_if_there_is_no_cycle_involving_the_last_element()
    {
        assert!(extract_canonical_cycle_from_last(&path(vec![7, 3, 2, 5, 8, 9, 5, 12])).is_none());
    }

    #[test]
    fn extract_canonical_cycle_from_last_should_return_a_cycle_involving_the_last_element_if_it_exists() {
        assert_eq!(
            extract_canonical_cycle_from_last(&path(vec![7, 3, 2, 5, 8, 9, 2])),
            cycle(vec![2, 5, 8, 9])
        );
        assert_eq!(
            extract_canonical_cycle_from_last(&path(vec![1, 3, 4, 5, 6, 1])),
            cycle(vec![1, 3, 4, 5, 6])
        );
    }

    #[test]
    fn length_of_cycle_is_number_of_vertices() {
        assert_eq!(cycle(vec![2, 3]).map(|c| c.len()), Some(2));
        assert_eq!(cycle(vec![2, 5, 8, 9]).map(|c| c.len()), Some(4));
        assert_eq!(cycle(vec![2, 5, 8, 1, 3, 4]).map(|c| c.len()), Some(6));
    }

    #[test]
    fn cycle_iterator_should_return_all_cycles_in_graph_1() {
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

        assert_eq!(cycle_iter(&g).count(), 1);
        assert_eq!(
            cycle_iter(&g).next().map(|x| x.canonical()),
            cycle(vec![5, 11, 10])
        );
    }

    #[test]
    fn cycle_iterator_should_return_all_cycles_in_graph_2() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 7));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 7));
        g.add_edge(edge(5, 6));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(6, 1));
        g.add_edge(edge(6, 3));

        assert_eq!(cycle_iter(&g).count(), 2);
        let cycles = cycle_iter(&g)
            .map(|c| c.canonical())
            .sorted()
            .collect::<Vec<Cycle>>();

        assert_eq!(
            cycles,
            vec![
                cycle(vec![1, 3, 4, 5, 6]).unwrap(),
                cycle(vec![3, 4, 5, 6]).unwrap(),
            ]
        );
    }

    #[test]
    fn cycle_iterator_should_return_all_cycles_in_graph_3() {
        let mut g = DirectedGraph::new();
        g.add_edge(edge(1, 2));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 1));
        g.add_edge(edge(2, 4));
        g.add_edge(edge(5, 3));

        assert_eq!(cycle_iter(&g).count(), 3);
        let cycles = cycle_iter(&g)
            .map(|c| c.canonical())
            .sorted()
            .collect::<Vec<Cycle>>();

        assert_eq!(
            cycles,
            vec![
                cycle(vec![1, 2, 3, 4, 5]).unwrap(),
                cycle(vec![1, 2, 4, 5]).unwrap(),
                cycle(vec![3, 4, 5]).unwrap(),
            ]
        );
    }

    // !!! Documenting behavior !!!
    #[test]
    fn cycle_iterator_does_not_work_on_a_all_disconnected_graph() {
        let mut g = DirectedGraph::new();
        // First component
        g.add_edge(edge(1, 3));
        g.add_edge(edge(3, 4));
        g.add_edge(edge(4, 7));
        g.add_edge(edge(4, 5));
        g.add_edge(edge(5, 7));
        g.add_edge(edge(5, 6));
        g.add_edge(edge(2, 3));
        g.add_edge(edge(6, 1));
        g.add_edge(edge(6, 3));
        // Second component, no vertex with no inbound edges
        g.add_edge(edge(10, 20));
        g.add_edge(edge(20, 30));
        g.add_edge(edge(30, 40));
        g.add_edge(edge(40, 50));
        g.add_edge(edge(50, 10));
        g.add_edge(edge(20, 40));
        g.add_edge(edge(50, 30));

        // !!! Finds only cycle from the first connected component !!!
        assert_eq!(cycle_iter(&g).count(), 2);
        let cycles = cycle_iter(&g)
            .map(|c| c.canonical())
            .sorted()
            .collect::<Vec<Cycle>>();
        assert_eq!(
            cycles,
            vec![
                cycle(vec![1, 3, 4, 5, 6]).unwrap(),
                cycle(vec![3, 4, 5, 6]).unwrap(),
            ]
        );
    }

    // Helpers

    fn vertex(id: u64) -> VertexId {
        VertexId(id)
    }

    fn edge(src: u64, dst: u64) -> Edge {
        Edge(vertex(src), vertex(dst))
    }

    fn vertices(ids: Vec<u64>) -> Vec<VertexId> {
        ids.iter().map(|id| vertex(*id)).collect()
    }

    fn cycle(ids: Vec<u64>) -> Option<Cycle> {
        Cycle::from_vertices(&vertices(ids))
    }

    fn cano_cycle(ids: Vec<u64>) -> Option<Cycle> {
        cycle(ids).map(|c| c.canonical())
    }

    fn path(ids: Vec<u64>) -> Path {
        Path::from(&vertices(ids))
    }
}
