use crate::directed_graph::DirectedGraph;
use crate::graph::Edge;
use crate::graph::VertexId;
use crate::iter::iter_datastructure::{Queue, SearchQueue};
use std::collections::HashMap;

type Flow = u64;
type Capacity = u64;
type ResidualCapacity = u64;

/// Direction of an edge in an Augmenting path
#[derive(Copy, PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    Forward,
    Backward,
}

/// An augmenting path is a path that have a residual capacity
/// It cannot be empty
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct AugmentingPath {
    path: Vec<(VertexId, Direction)>,
}

impl AugmentingPath {
    fn new(start: VertexId) -> AugmentingPath {
        AugmentingPath {
            path: vec![(start, Direction::Forward)],
        }
    }

    fn edges(&self) -> impl Iterator<Item = (Edge, Direction)> + '_ {
        let tail = self.path.iter().skip(1);
        self.path
            .iter()
            .zip(tail)
            .map(|((src, _), (dst, dir))| (Edge(*src, *dst), *dir))
    }

    fn append(&self, vid: VertexId, direction: Direction) -> AugmentingPath {
        let mut cloned = self.path.clone();
        cloned.push((vid, direction));
        AugmentingPath { path: cloned }
    }

    fn contains_vertex(&self, vid: VertexId) -> bool {
        self.path.iter().find(|(v, _)| vid == *v).is_some()
    }

    fn last(&self) -> Option<&(VertexId, Direction)> {
        self.path.last()
    }
}

/// Maximum flow
/// Implementation of Ford-Fulkerson algorithm
pub fn max_flow<CFn>(
    graph: &DirectedGraph,
    capacity: CFn,
    start: VertexId,
    end: VertexId,
) -> (Flow, HashMap<Edge, (Flow, Capacity)>)
where
    CFn: Fn(&Edge) -> Capacity,
{
    // Initialising current flow to 0 everywhere
    let mut current_flow: HashMap<Edge, (Flow, Capacity)> = HashMap::new();
    for edge in graph.edges() {
        current_flow.insert(*edge, (0, capacity(edge)));
    }
    let mut max_flow = 0;
    // As long a we can find an augmenting path,
    // we can update the flow along the path using the residual capacity
    while let Some((path, residual_cap)) = find_augmenting_path(&graph, &current_flow, start, end) {
        update_flow(&mut current_flow, path, residual_cap);
        max_flow = max_flow + residual_cap;
    }

    (max_flow, current_flow)
}

/// Finds an Augmenting Path
fn find_augmenting_path(
    graph: &DirectedGraph,
    current_flow: &HashMap<Edge, (Flow, Capacity)>,
    start: VertexId,
    target: VertexId,
) -> Option<(AugmentingPath, ResidualCapacity)> {
    // !!! Highly inefficent !!!
    // But using the iterator directly would borrow the current flow as immuatble,
    // when a mutable borrow would be needed to update the map afterwards ...
    iter_augmenting_path(graph, &current_flow, start, target).next()
}

/// Iter on augmenting paths
fn iter_augmenting_path<'a>(
    graph: &'a DirectedGraph,
    current_flow: &'a HashMap<Edge, (Flow, Capacity)>,
    start: VertexId,
    target: VertexId,
) -> AugmentingPathIter<'a> {
    AugmentingPathIter::new(graph, current_flow, start, target)
}

fn update_flow(
    current_flow: &mut HashMap<Edge, (Flow, Capacity)>,
    path: AugmentingPath,
    delta: ResidualCapacity,
) {
    use Direction::*;
    for (edge, direction) in path.edges() {
        match direction {
            Forward => current_flow
                .entry(edge)
                .and_modify(|flow_cap| flow_cap.0 += delta),
            Backward => current_flow
                .entry(edge.reverse())
                .and_modify(|flow_cap| flow_cap.0 -= delta),
        };
    }
}

/// Computes the residual capacity of an Augmenting Path
fn residual_capacity(
    path: &AugmentingPath,
    current_flow: &HashMap<Edge, (Flow, Capacity)>,
) -> ResidualCapacity {
    path.edges()
        .map(|(e, dir)| edge_residual_capacity(&e, &dir, current_flow))
        .min()
        .expect("An augmenting path cannot be empty")
}

/// Computes the residual capacity of an edge
fn edge_residual_capacity(
    edge: &Edge,
    direction: &Direction,
    current_flow: &HashMap<Edge, (Flow, Capacity)>,
) -> ResidualCapacity {
    use Direction::*;
    match direction {
        Forward => {
            let (flow, cap) = current_flow
                .get(edge)
                .expect("All edges must have been initialized already");
            *cap - *flow
        }
        Backward => {
            let (flow, _) = current_flow
                .get(&(*edge).reverse())
                .expect("All edges must have been initialized already");
            *flow
        }
    }
}

/// Breadth-First Augmenting Path iterator

struct AugmentingPathIter<'a> {
    queue: Queue<AugmentingPath>,
    graph: &'a DirectedGraph,
    // FIXME: This map needs to be updated externally to the iterator during iteration
    // This breaks borrowing rules
    current_flow: &'a HashMap<Edge, (Flow, Capacity)>,
    target: VertexId,
}

impl<'a> AugmentingPathIter<'a> {
    fn new(
        graph: &'a DirectedGraph,
        current_flow: &'a HashMap<Edge, (Flow, Capacity)>,
        start: VertexId,
        target: VertexId,
    ) -> AugmentingPathIter<'a> {
        let mut queue: Queue<AugmentingPath> = Queue::<Queue<AugmentingPath>>::new();
        queue.push(AugmentingPath::new(start));
        AugmentingPathIter {
            queue,
            graph,
            current_flow,
            target,
        }
    }
}

impl<'a> Iterator for AugmentingPathIter<'a> {
    type Item = (AugmentingPath, ResidualCapacity);
    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;

        // Looping until we have a complete path to the target
        while let Some(path) = self.queue.pop() {
            let (vid, _) = path
                .last()
                .expect("We shouldn't never have any empty path in the queue !");

            let foward_edges = self
                .graph
                .outbound_edges(*vid)
                // .filter(|e| !is_full(e))
                .map(|Edge(_, v)| (v, Forward));
            let backward_edges = self
                .graph
                .inbound_edges(*vid)
                .map(|Edge(_, v)| (v, Backward));
            // forward + backward edges
            (foward_edges.chain(backward_edges))
                .filter(|(v, _)| !path.contains_vertex(**v)) // avoiding cycles here
                .for_each(|(v, dir)| self.queue.push(path.append(*v, dir)));

            // Path has reach the target vertex
            if *vid == self.target {
                // If the residual capacity is positive => we return the path, as we can augment the flow along the path
                // else we continue looking for other paths
                let residual_cap = residual_capacity(&path, self.current_flow);
                if residual_cap > 0 {
                    return Some((path, residual_cap));
                }
            }
        }

        // FIXME:
        // Here should we restart the search in case we can now find some more augmenting paths ?
        // Or is it guaranteed that we have increased the flow to the max capacity here ?
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directed_graph::DirectedGraph;

    #[test]
    fn max_flow_should_compute_maximum_flow_in_a_simple_graph_1() {
        let (g, capfn) = build_simple_test_flow_1();
        let (max, flows) = max_flow(&g, capfn, VertexId(0), VertexId(5));
        let flow_for =
            move |v1: u64, v2: u64| -> (Flow, Capacity) { *flows.get(&edge(v1, v2)).unwrap() };

        // Max Flow is 23
        assert_eq!(max, 23);
        // Individual flow per edges
        assert_eq!(flow_for(0, 1), (12, 16));
        assert_eq!(flow_for(1, 3), (12, 12));
        assert_eq!(flow_for(3, 5), (19, 20));
        assert_eq!(flow_for(0, 2), (11, 13));
        assert_eq!(flow_for(2, 1), (0, 4));
        assert_eq!(flow_for(2, 4), (11, 14));
        assert_eq!(flow_for(4, 3), (7, 7));
        assert_eq!(flow_for(4, 5), (4, 4));
        assert_eq!(flow_for(1, 2), (0, 10));
        assert_eq!(flow_for(3, 2), (0, 9));
    }

    #[test]
    fn max_flow_should_compute_maximum_flow_in_a_simple_graph_2() {
        let (g, capfn) = build_simple_test_flow_2();
        let (max, flows) = max_flow(&g, capfn, VertexId(0), VertexId(7));
        let flow_for =
            move |v1: u64, v2: u64| -> (Flow, Capacity) { *flows.get(&edge(v1, v2)).unwrap() };

        // Max Flow is 22
        assert_eq!(max, 22);
        assert_eq!(flow_for(0, 1), (8, 8));
        assert_eq!(flow_for(0, 2), (9, 9));
        assert_eq!(flow_for(0, 3), (5, 7));
        assert_eq!(flow_for(1, 4), (2, 2));
        assert_eq!(flow_for(1, 5), (6, 6));
        assert_eq!(flow_for(2, 4), (4, 4));
        assert_eq!(flow_for(2, 5), (1, 6));
        assert_eq!(flow_for(2, 6), (4, 4));
        assert_eq!(flow_for(3, 5), (0, 1));
        assert_eq!(flow_for(3, 6), (5, 5));
        assert_eq!(flow_for(4, 7), (6, 8));
        assert_eq!(flow_for(5, 7), (7, 7));
        assert_eq!(flow_for(6, 7), (9, 9));
    }

    #[test]
    fn max_flow_should_compute_maximum_flow_in_a_simple_graph_3() {
        let (g, capfn) = build_simple_test_flow_3();
        let (max, flows) = max_flow(&g, capfn, VertexId(0), VertexId(7));
        let flow_for =
            move |v1: u64, v2: u64| -> (Flow, Capacity) { *flows.get(&edge(v1, v2)).unwrap() };

        // Max Flow is 28
        assert_eq!(max, 28);
        assert_eq!(flow_for(0, 1), (10, 10));
        assert_eq!(flow_for(0, 2), (5, 5));
        assert_eq!(flow_for(0, 3), (13, 15));
        assert_eq!(flow_for(1, 2), (0, 4));
        assert_eq!(flow_for(1, 4), (9, 9));
        assert_eq!(flow_for(1, 5), (1, 15));
        assert_eq!(flow_for(2, 3), (0, 4));
        assert_eq!(flow_for(2, 5), (8, 8));
        assert_eq!(flow_for(3, 6), (13, 16));
        assert_eq!(flow_for(4, 7), (9, 10));
        assert_eq!(flow_for(4, 5), (0, 15));
        assert_eq!(flow_for(5, 6), (0, 15));
        assert_eq!(flow_for(5, 7), (9, 10));
        assert_eq!(flow_for(6, 2), (3, 6));
        assert_eq!(flow_for(6, 7), (10, 10));
    }

    // Helpers

    // Max flow in this test graph should be 23
    // Graph taken from https://www.geeksforgeeks.org/max-flow-problem-introduction/
    fn build_simple_test_flow_1() -> (DirectedGraph, impl Fn(&Edge) -> Capacity) {
        let mut g = DirectedGraph::new();
        let mut capacity: HashMap<Edge, Capacity> = HashMap::new();
        cap_edge(&mut g, &mut capacity, 0, 1, 16);
        cap_edge(&mut g, &mut capacity, 0, 2, 13);
        cap_edge(&mut g, &mut capacity, 1, 3, 12);
        cap_edge(&mut g, &mut capacity, 1, 2, 10);
        cap_edge(&mut g, &mut capacity, 2, 1, 4);
        cap_edge(&mut g, &mut capacity, 2, 4, 14);
        cap_edge(&mut g, &mut capacity, 3, 5, 20);
        cap_edge(&mut g, &mut capacity, 3, 2, 9);
        cap_edge(&mut g, &mut capacity, 4, 3, 7);
        cap_edge(&mut g, &mut capacity, 4, 5, 4);

        let capfn = move |e: &Edge| -> Capacity { *capacity.get(e).unwrap_or(&0) };
        (g, capfn)
    }

    // Max flow in this test graph should be 23
    // Graph taken from Graph, Algorithms and Optimisation, p194
    fn build_simple_test_flow_2() -> (DirectedGraph, impl Fn(&Edge) -> Capacity) {
        let mut g = DirectedGraph::new();
        let mut capacity: HashMap<Edge, Capacity> = HashMap::new();
        cap_edge(&mut g, &mut capacity, 0, 1, 8);
        cap_edge(&mut g, &mut capacity, 0, 2, 9);
        cap_edge(&mut g, &mut capacity, 0, 3, 7);
        cap_edge(&mut g, &mut capacity, 1, 4, 2);
        cap_edge(&mut g, &mut capacity, 1, 5, 6);
        cap_edge(&mut g, &mut capacity, 2, 4, 4);
        cap_edge(&mut g, &mut capacity, 2, 5, 6);
        cap_edge(&mut g, &mut capacity, 2, 6, 4);
        cap_edge(&mut g, &mut capacity, 3, 5, 1);
        cap_edge(&mut g, &mut capacity, 3, 6, 5);
        cap_edge(&mut g, &mut capacity, 4, 7, 8);
        cap_edge(&mut g, &mut capacity, 5, 7, 7);
        cap_edge(&mut g, &mut capacity, 6, 7, 9);

        let capfn = move |e: &Edge| -> Capacity { *capacity.get(e).unwrap_or(&0) };
        (g, capfn)
    }

    fn build_simple_test_flow_3() -> (DirectedGraph, impl Fn(&Edge) -> Capacity) {
        let mut g = DirectedGraph::new();
        let mut capacity: HashMap<Edge, Capacity> = HashMap::new();
        cap_edge(&mut g, &mut capacity, 0, 1, 10);
        cap_edge(&mut g, &mut capacity, 0, 2, 5);
        cap_edge(&mut g, &mut capacity, 0, 3, 15);
        cap_edge(&mut g, &mut capacity, 1, 2, 4);
        cap_edge(&mut g, &mut capacity, 1, 4, 9);
        cap_edge(&mut g, &mut capacity, 1, 5, 15);
        cap_edge(&mut g, &mut capacity, 2, 3, 4);
        cap_edge(&mut g, &mut capacity, 2, 5, 8);
        cap_edge(&mut g, &mut capacity, 3, 6, 16);
        cap_edge(&mut g, &mut capacity, 4, 5, 15);
        cap_edge(&mut g, &mut capacity, 4, 7, 10);
        cap_edge(&mut g, &mut capacity, 5, 6, 15);
        cap_edge(&mut g, &mut capacity, 5, 7, 10);
        cap_edge(&mut g, &mut capacity, 6, 2, 6);
        cap_edge(&mut g, &mut capacity, 6, 7, 10);

        let capfn = move |e: &Edge| -> Capacity { *capacity.get(e).unwrap_or(&0) };
        (g, capfn)
    }

    fn edge(src: u64, dst: u64) -> Edge {
        Edge(VertexId(src), VertexId(dst))
    }

    fn cap_edge(
        g: &mut DirectedGraph,
        capacity: &mut HashMap<Edge, Capacity>,
        src: u64,
        dst: u64,
        cap: Capacity,
    ) {
        g.add_edge(edge(src, dst));
        capacity.insert(edge(src, dst), cap);
    }
}
