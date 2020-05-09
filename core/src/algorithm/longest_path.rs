use crate::algorithm::shortest_path::dag_shortest_paths;
use crate::algorithm::topo_sort::DAG;
use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};
use crate::path::{Path, ScoredPath};
use std::collections::hash_map::HashMap;

/// Finds the longest path from a source to a target vertex in a DAG
pub fn dag_longest_path<F>(
    dag: DAG,
    scorefn: F,
    start: VertexId,
    end: VertexId,
) -> Option<ScoredPath>
where
    F: Fn(&Edge) -> i64,
{
    dag_longest_paths(dag, scorefn, start).remove(&end)
}

/// Finds the longest path from a source to all reachable vertices in a DAG
/// This reusing dag_shortest_paths and works by negating all scores.
pub fn dag_longest_paths<F>(dag: DAG, scorefn: F, start: VertexId) -> HashMap<VertexId, ScoredPath>
where
    F: Fn(&Edge) -> i64,
{
    let negated_score_fn = |e: &Edge| -1 * scorefn(e);
    negate_scores(dag_shortest_paths(dag, negated_score_fn, start))
}

fn negate_scores(mut scores: HashMap<VertexId, ScoredPath>) -> HashMap<VertexId, ScoredPath> {
    for (_, sp) in &mut scores {
        sp.score = -1 * sp.score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::topo_sort;

    #[test]
    fn dag_longest_path_should_find_the_longest_path_in_a_dag() {
        let (g, scorefn) = build_test_weighted_graph();
        let shortest_path = dag_longest_path(
            topo_sort::try_dag(&g).unwrap(),
            scorefn,
            VertexId(1),
            VertexId(8),
        );
        assert_eq!(shortest_path, Some(scored_path_of(23, vec![1, 2, 5, 8])));
    }

    #[test]
    fn dag_longest_paths_should_find_all_longest_paths_from_source_vertex_in_a_dag() {
        let (g, scorefn) = build_test_weighted_graph();

        let all_longest_paths_from_1 =
            dag_longest_paths(topo_sort::try_dag(&g).unwrap(), scorefn, VertexId(1));

        assert_eq!(all_longest_paths_from_1.len(), 8);
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(1)).unwrap(),
            &scored_path_of(0, vec![1]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(2)).unwrap(),
            &scored_path_of(3, vec![1, 2]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(3)).unwrap(),
            &scored_path_of(7, vec![1, 2, 3]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(4)).unwrap(),
            &scored_path_of(15, vec![1, 2, 3, 4]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(5)).unwrap(),
            &scored_path_of(14, vec![1, 2, 5]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(6)).unwrap(),
            &scored_path_of(20, vec![1, 2, 3, 4, 6]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(7)).unwrap(),
            &scored_path_of(18, vec![1, 2, 3, 7]),
        );
        assert_eq!(
            all_longest_paths_from_1.get(&VertexId(8)).unwrap(),
            &scored_path_of(23, vec![1, 2, 5, 8]),
        );
    }

    // Helpers

    // Graph taken from https://www.youtube.com/watch?v=TXkDpqjDMHA
    fn build_test_weighted_graph() -> (DirectedGraph, impl Fn(&Edge) -> i64) {
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
