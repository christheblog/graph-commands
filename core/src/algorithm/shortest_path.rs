use crate::algorithm::topo_sort;
use crate::algorithm::topo_sort::DAG;
use crate::directed_graph::DirectedGraph;
use crate::graph::Edge;
use crate::graph::VertexId;
use crate::iter::iter_datastructure::{Queue, SearchQueue};
use crate::path::{Path, ScoredPath};
use std::collections::hash_map::HashMap;

/// Finds the sortest path from a source to a target vertex in a DAG
pub fn dag_shortest_path<F>(
    dag: DAG,
    scorefn: F,
    start: VertexId,
    end: VertexId,
) -> Option<ScoredPath>
where
    F: Fn(&Edge) -> i64,
{
    dag_shortest_paths(dag, scorefn, start).remove(&end)
}

/// Finds the sortest path from a source to all reachable vertices in a DAG
pub fn dag_shortest_paths<F>(dag: DAG, scorefn: F, start: VertexId) -> HashMap<VertexId, ScoredPath>
where
    F: Fn(&Edge) -> i64,
{
    let graph = dag.as_graph();
    let topo_order =
        topo_sort::topological_sort(graph).expect("A DAG should have a topological order !");
    let mut scores: HashMap<VertexId, ScoredPath> = HashMap::new();
    scores.insert(start, scored_path_of_one(start, 0));

    topo_order
        .iter()
        .skip_while(|v| **v != start)
        .for_each(|v| {
            graph.outbound_edges(*v).for_each(|edge| {
                // FIXME is it necessarily true here ...
                let ScoredPath { path, score } = scores
                    .get(v)
                    .expect("The current score of the processed vertex should be in the map");
                let Edge(_, w) = edge;
                let new_score = score + scorefn(&edge);
                if let Some(current_score) = scores.get(w).map(|x| x.score) {
                    // we found a shorter path to w => we update
                    if new_score < current_score {
                        let new_path = path.append(*w);
                        scores.insert(*w, scored_path_of(new_path, new_score));
                    }
                } else {
                    let new_path = path.append(*w);
                    scores.insert(*w, scored_path_of(new_path, new_score));
                };
            });
        });
    scores
}

/// Bellman-Ford implementation - can handle negative weights
/// In case a negative cycle is detected, algorithm will return None
/// Implements algorithm as descrived in https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm
pub fn bellman_ford<F>(
    graph: &DirectedGraph,
    scorefn: F,
    start: VertexId,
) -> Option<HashMap<VertexId, ScoredPath>>
where
    F: Fn(&Edge) -> i64,
{
    let mut scores: HashMap<VertexId, ScoredPath> = HashMap::new();
    scores.insert(start, scored_path_of_one(start, 0));

    for _ in 1..graph.vertex_count() {
        for edge in graph.edges() {
            let weight = scorefn(edge);
            let Edge(u, v) = edge;
            if let Some(ScoredPath { path, score }) = scores.get(&u) {
                if *score + weight < current_score_of_vertex(&scores, v) {
                    let (new_path, new_score) = (path.append(*v), *score + weight);
                    scores.insert(*v, scored_path_of(new_path, new_score));
                }
            }
        }
    }

    // Looking for cycle with negative weights.
    // Iterating a last time on all edges.
    // If a path can actually be improved after having iterated |V| -1, this means there is a negative cycle
    for edge in graph.edges() {
        let weight = scorefn(edge);
        let Edge(u, v) = edge;
        if let Some(ScoredPath { score, .. }) = scores.get(&u) {
            if *score + weight < current_score_of_vertex(&scores, v) {
                return None;
            }
        }
    }

    // If we reached that point, no negative cycles have been detected
    Some(scores)
}

/// SPFA - Shortest Path Faster Algorithm
/// This is an improvement to Bellman-Ford using a queue to avoid blind scanning of edges - but does not terminate on negative weight cycles
/// Implements algorithm as descrived in https://en.wikipedia.org/wiki/Shortest_Path_Faster_Algorithm
pub fn spfa<F>(graph: &DirectedGraph, scorefn: F, start: VertexId) -> HashMap<VertexId, ScoredPath>
where
    F: Fn(&Edge) -> i64,
{
    let mut scores: HashMap<VertexId, ScoredPath> = HashMap::new();
    scores.insert(start, scored_path_of_one(start, 0));

    // Using a standard FIFO queue
    let mut q: Queue<VertexId> = Queue::<VertexId>::new();
    q.push(start);

    while let Some(vid) = q.pop() {
        for edge in graph.outbound_edges(vid) {
            let weight = scorefn(edge);
            let Edge(u, v) = edge;
            if let Some(ScoredPath { path, score }) = scores.get(&u) {
                if *score + weight < current_score_of_vertex(&scores, v) {
                    let (new_path, new_score) = (path.append(*v), *score + weight);
                    scores.insert(*v, scored_path_of(new_path, new_score));
                    q.push(*v);
                }
            }
        }
    }

    scores
}

// Helpers

fn scored_path_of_one(v: VertexId, score: i64) -> ScoredPath {
    ScoredPath {
        path: Path::from(&vec![v]),
        score: score,
    }
}

fn scored_path_of(path: Path, score: i64) -> ScoredPath {
    ScoredPath {
        path: path,
        score: score,
    }
}

// Extract current score for a given vertex from a scored path map, or i64::MAX if there is no score yet
fn current_score_of_vertex(scores: &HashMap<VertexId, ScoredPath>, vid: &VertexId) -> i64 {
    scores
        .get(vid)
        .map(|sp| sp.score)
        .unwrap_or_else(|| std::i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Shortest-path in a DAG

    #[test]
    fn dag_shortest_path_should_find_the_shortest_path_in_a_dag() {
        let (g, scorefn) = build_test_weighted_dag();
        let shortest_path = dag_shortest_path(
            topo_sort::try_dag(&g).unwrap(),
            scorefn,
            VertexId(1),
            VertexId(8),
        );
        assert_eq!(shortest_path, Some(scored_path_of(11, vec![1, 2, 4, 7, 8])));
    }

    #[test]
    fn dag_shortest_paths_should_find_all_shortest_paths_from_source_vertex_in_a_dag() {
        let (g, scorefn) = build_test_weighted_dag();

        let all_shortest_paths_from_1 =
            dag_shortest_paths(topo_sort::try_dag(&g).unwrap(), scorefn, VertexId(1));

        assert_eq!(all_shortest_paths_from_1.len(), 8);
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(1)).unwrap(),
            &scored_path_of(0, vec![1]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(2)).unwrap(),
            &scored_path_of(3, vec![1, 2]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(3)).unwrap(),
            &scored_path_of(6, vec![1, 3]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(4)).unwrap(),
            &scored_path_of(7, vec![1, 2, 4]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(5)).unwrap(),
            &scored_path_of(3, vec![1, 2, 4, 5]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(6)).unwrap(),
            &scored_path_of(12, vec![1, 2, 4, 6]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(7)).unwrap(),
            &scored_path_of(9, vec![1, 2, 4, 7]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(8)).unwrap(),
            &scored_path_of(11, vec![1, 2, 4, 7, 8]),
        );
    }

    // Bellman-Ford-Moore

    #[test]
    fn bfm_shortest_paths_should_find_all_shortest_paths_from_source_vertex_in_a_dag() {
        let (g, scorefn) = build_test_weighted_dag();

        let all_shortest_paths_from_1 = bellman_ford(&g, scorefn, VertexId(1))
            .expect("A DAG cannot have a negative weight cycle");

        assert_eq!(all_shortest_paths_from_1.len(), 8);
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(1)).unwrap(),
            &scored_path_of(0, vec![1]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(2)).unwrap(),
            &scored_path_of(3, vec![1, 2]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(3)).unwrap(),
            &scored_path_of(6, vec![1, 3]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(4)).unwrap(),
            &scored_path_of(7, vec![1, 2, 4]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(5)).unwrap(),
            &scored_path_of(3, vec![1, 2, 4, 5]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(6)).unwrap(),
            &scored_path_of(12, vec![1, 2, 4, 6]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(7)).unwrap(),
            &scored_path_of(9, vec![1, 2, 4, 7]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(8)).unwrap(),
            &scored_path_of(11, vec![1, 2, 4, 7, 8]),
        );
    }

    // SPFA

    #[test]
    fn spfa_shortest_paths_should_find_all_shortest_paths_from_source_vertex_in_a_dag() {
        let (g, scorefn) = build_test_weighted_dag();

        let all_shortest_paths_from_1 = spfa(&g, scorefn, VertexId(1));

        assert_eq!(all_shortest_paths_from_1.len(), 8);
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(1)).unwrap(),
            &scored_path_of(0, vec![1]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(2)).unwrap(),
            &scored_path_of(3, vec![1, 2]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(3)).unwrap(),
            &scored_path_of(6, vec![1, 3]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(4)).unwrap(),
            &scored_path_of(7, vec![1, 2, 4]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(5)).unwrap(),
            &scored_path_of(3, vec![1, 2, 4, 5]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(6)).unwrap(),
            &scored_path_of(12, vec![1, 2, 4, 6]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(7)).unwrap(),
            &scored_path_of(9, vec![1, 2, 4, 7]),
        );
        assert_eq!(
            all_shortest_paths_from_1.get(&VertexId(8)).unwrap(),
            &scored_path_of(11, vec![1, 2, 4, 7, 8]),
        );
    }

    // Helpers

    // Graph taken from https://www.youtube.com/watch?v=TXkDpqjDMHA
    fn build_test_weighted_dag() -> (DirectedGraph, impl Fn(&Edge) -> i64) {
        let mut g = DirectedGraph::new();
        let mut weights: HashMap<Edge, i64> = HashMap::new();
        weighted_edge(&mut g, &mut weights, 1, 2, 3);
        weighted_edge(&mut g, &mut weights, 1, 3, 6);
        weighted_edge(&mut g, &mut weights, 2, 3, 4);
        weighted_edge(&mut g, &mut weights, 2, 4, 4);
        weighted_edge(&mut g, &mut weights, 2, 5, 11);
        weighted_edge(&mut g, &mut weights, 3, 4, 8);
        weighted_edge(&mut g, &mut weights, 4, 5, -4);
        weighted_edge(&mut g, &mut weights, 3, 7, 11);
        weighted_edge(&mut g, &mut weights, 4, 6, 5);
        weighted_edge(&mut g, &mut weights, 4, 7, 2);
        weighted_edge(&mut g, &mut weights, 5, 8, 9);
        weighted_edge(&mut g, &mut weights, 6, 8, 1);
        weighted_edge(&mut g, &mut weights, 7, 8, 2);

        let scorefn = move |e: &Edge| -> i64 { *weights.get(e).unwrap() };
        (g, scorefn)
    }

    fn edge(src: u64, dst: u64) -> Edge {
        Edge(VertexId(src), VertexId(dst))
    }

    fn weighted_edge(
        g: &mut DirectedGraph,
        weights: &mut HashMap<Edge, i64>,
        src: u64,
        dst: u64,
        w: i64,
    ) {
        g.add_edge(edge(src, dst));
        weights.insert(edge(src, dst), w);
    }

    fn scored_path_of(score: i64, vertices: Vec<u64>) -> ScoredPath {
        ScoredPath {
            path: Path {
                vertices: vertices.iter().map(|x| VertexId(*x)).collect(),
            },
            score: score,
        }
    }
}
