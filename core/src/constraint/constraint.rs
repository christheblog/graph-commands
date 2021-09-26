use crate::graph::Edge;
use crate::graph::VertexId;
use crate::path::Path;
use crate::path::ScoredPath;

type ConstraintRef = Box<Constraint>;

/// Constraints that can be applied to a ScoredPath
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// Ensure the path contains the given VertexId
    ContainsVertex(VertexId),
    /// Ensure the path contains the given Edge
    ContainsEdge(Edge),
    /// Ensures the order in which vertices are appearing in the path a respecting the provided order
    /// Note:
    /// All vertices in the OrderedVertices don't have to appear. But if they appear, they do have to be in teh right order
    OrderedVertices(Vec<VertexId>),
    // Contains a Cycle
    ContainsCycle,
    /// Ensure the path has a minimum length
    MinLength(usize),
    /// Ensure the path has a maximum length
    MaxLength(usize),
    /// Ensure the path has a minimum score
    MinScore(i64),
    /// Ensure the path has a maximum score
    MaxScore(i64),
    /// Ensure that at least one of the constraints is satified
    Or(ConstraintRef, ConstraintRef),
    /// Ensure one or the other the constraint is satified
    Xor(ConstraintRef, ConstraintRef),
    /// Ensure both constraints are satified
    And(ConstraintRef, ConstraintRef),
    /// Ensure the constraint is not satisfied
    Not(ConstraintRef),
    // TODO could a Custom constraint to support user implemented constraints
    // But this will require dynamic dispatch :
    // Custom(Box<dyn Fn(ScoredPath) -> bool>, Box<dyn Fn(ScoredPath) -> bool>)
}

impl Constraint {
    /// Check applied on a PARTIAL Path
    /// This check MUST return true IF the constraint has still a chance to be met later on
    /// (ie the Path will have more vertices added to it, and score will be increased)
    pub fn check_partial(&self, partial: &ScoredPath) -> bool {
        use Constraint::*;
        match self {
            ContainsVertex(_) | ContainsEdge(_) => true,
            OrderedVertices(ordered) => Constraint::check_vertices_order(&partial.path, ordered),
            ContainsCycle => true,
            MinLength(_) | MinScore(_) => true,
            MaxLength(len) => partial.path.size() <= *len,
            MaxScore(score) => partial.score <= *score,
            Not(x) => match **x {
                // Optimisations for partial paths that can be rejected straight away when negated
                ContainsVertex(vid) => !partial.path.contains_vertex(&vid),
                ContainsEdge(edge) => !partial.path.contains_edge(&edge),
                ContainsCycle => !partial.path.contains_cycle(),
                // "In general", Partial Not can still meet their requirement later if constraint is true => always accept them
                _ => true,
            },
            // Or can still be met if constraint 1 OR constraint 2 have still a chance to be met
            Or(c1, c2) => c1.check_partial(partial) || c2.check_partial(partial),
            // And can be met only if both constraints still have a chance to be met
            And(c1, c2) => c1.check_partial(partial) && c2.check_partial(partial),
            // Xor cannot be met only when none of the constraint can be met
            Xor(c1, c2) => c1.check_partial(partial) || c2.check_partial(partial),
        }
    }

    /// Check applied on a COMPLETE Path
    pub fn check_complete(&self, full: &ScoredPath) -> bool {
        use Constraint::*;
        match self {
            ContainsVertex(vid) => full.path.contains_vertex(vid),
            ContainsEdge(edge) => full.path.contains_edge(edge),
            OrderedVertices(ordered) => Constraint::check_vertices_order(&full.path, ordered),
            ContainsCycle => full.path.contains_cycle(),
            MinLength(len) => full.path.size() >= *len,
            MaxLength(len) => full.path.size() <= *len,
            MinScore(score) => full.score >= *score,
            MaxScore(score) => full.score <= *score,
            // Constraint combination
            Or(c1, c2) => c1.check_complete(full) || c2.check_complete(full),
            Xor(c1, c2) => c1.check_complete(full) ^ c2.check_complete(full),
            And(c1, c2) => c1.check_complete(full) && c2.check_complete(full),
            Not(c1) => !c1.check_complete(full),
        }
    }

    // Check the verices in the path appears by the specified order of ordered
    // Note: All vertices of ordered don't have to appear in the path
    fn check_vertices_order(path: &Path, ordered: &Vec<VertexId>) -> bool {
        use std::collections::HashMap;
        // Cache : vertex -> index in the vector (relative position index)
        let mut vertex_to_index = HashMap::with_capacity(ordered.len());
        for (index, vertex) in ordered.iter().enumerate() {
            vertex_to_index.insert(vertex, index);
        }
        let mut start_from = 0;
        for vertex in &path.vertices {
            if let Some(relative_index) = vertex_to_index.get(vertex) {
                if *relative_index < start_from {
                    return false;
                } else {
                    start_from = *relative_index;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Constraint::*;

    // Partial contains vertex should always be true
    // because the vertex can alwyas be added later on
    #[test]
    fn partial_contains_vertex_should_always_be_true() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_partial(&ContainsVertex(VertexId(3)), &path),
            true
        );
    }

    // Partial contains vertex should always be true
    // because the vertex can alwyas be added later on
    #[test]
    fn partial_contains_edge_should_always_be_true() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_partial(&ContainsEdge(Edge(VertexId(1), VertexId(3))), &path),
            true
        );
    }

    // Ordered vertices should detect when a path will not respect the order of the vertices
    #[test]
    fn partial_ordered_vertices_should_detect_invalid_path() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_partial(
                &OrderedVertices(vec![VertexId(2), VertexId(5), VertexId(1)]),
                &path
            ),
            false
        );
    }

    // Ordered vertices should allow a partial path that is respecting the order of the vertices so far
    #[test]
    fn partial_ordered_vertices_should_allow_correct_partial_path() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_partial(
                &OrderedVertices(vec![VertexId(1), VertexId(5), VertexId(2), VertexId(7)]),
                &path
            ),
            true
        );
    }

    // Cycle
    #[test]
    fn contains_cycle_should_always_be_true_on_partial_path() {
        let path = score_of(path_of(vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(Constraint::check_partial(&ContainsCycle, &path), true);
    }

    #[test]
    fn contains_cycle_should_be_true_on_partial_path_with_an_actual_cycle() {
        let path = score_of(path_of(vec![1, 2, 3, 4, 5, 2, 7, 8]), 1);
        assert_eq!(Constraint::check_partial(&ContainsCycle, &path), true);
    }

    // MinLength
    #[test]
    fn partial_min_length_should_allow_any_path() {
        let path = score_of(path_of(vec![1, 2, 3, 4]), 1);
        assert_eq!(path.path.size(), 4);
        assert_eq!(Constraint::check_partial(&MinLength(2), &path), true);
        assert_eq!(Constraint::check_partial(&MinLength(5), &path), true);
    }

    // MaxLength
    #[test]
    fn partial_max_length_should_allow_a_shorter_path() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(path.path.size(), 2);
        assert_eq!(Constraint::check_partial(&MaxLength(3), &path), true);
    }

    #[test]
    fn partial_max_score_should_reject_a_too_long_path() {
        let path = score_of(path_of(vec![1, 2, 3, 4]), 1);
        assert_eq!(path.path.size(), 4);
        assert_eq!(Constraint::check_partial(&MaxLength(3), &path), false);
    }

    // MinScore
    #[test]
    fn partial_min_score_should_allow_any_path() {
        let path = score_of(path_of(vec![1, 2]), 10);
        assert_eq!(Constraint::check_partial(&MinScore(12), &path), true);
        assert_eq!(Constraint::check_partial(&MinScore(5), &path), true);
    }

    // MaxScore
    #[test]
    fn partial_max_length_should_allow_a_smaller_score() {
        let path = score_of(path_of(vec![1, 2]), 10);
        assert_eq!(Constraint::check_partial(&MaxScore(12), &path), true);
    }

    #[test]
    fn partial_max_score_should_reject_a_too_high_score() {
        let path = score_of(path_of(vec![1, 2]), 25);
        assert_eq!(Constraint::check_partial(&MaxScore(12), &path), false);
    }

    // Or combinator
    #[test]
    fn partial_or_should_be_true_if_any_of_the_constraints_can_still_be_met() {
        let path = score_of(path_of(vec![1, 2]), 1);
        let constraint = Or(box_of(MaxScore(12)), box_of(MaxScore(15)));
        assert_eq!(Constraint::check_partial(&constraint, &path), true);
    }

    #[test]
    fn partial_or_should_be_false_if_none_of_the_constraints_can_be_met() {
        let path = score_of(path_of(vec![1, 2]), 20);
        assert_eq!(
            Constraint::check_partial(&Or(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            false
        );
    }

    // And combinator
    #[test]
    fn partial_and_should_be_true_if_both_constraints_can_still_be_met() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_partial(&And(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            true
        );
    }

    #[test]
    fn partial_and_should_be_false_if_any_of_the_constraint_cannot_be_met() {
        let path = score_of(path_of(vec![1, 2]), 13);
        assert_eq!(
            Constraint::check_partial(&And(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            false
        );
    }

    // Xor combinator

    // Partial Xor: should be true if one or both of the constraint can still be met
    // If both constraint can be met, this is because one can ultimately not be met
    // therefore the final Xor constraint check could be true
    #[test]
    fn partial_xor_should_be_true_if_both_constraints_can_still_be_met() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_partial(&Xor(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            true
        );
    }

    #[test]
    fn partial_xor_should_be_true_if_one_of_the_constraints_can_still_be_met() {
        let path = score_of(path_of(vec![1, 2]), 13);
        assert_eq!(
            Constraint::check_partial(&Xor(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            true
        );
    }

    #[test]
    fn partial_xor_should_be_false_if_both_the_constraints_cannot_be_met() {
        let path = score_of(path_of(vec![1, 2]), 16);
        assert_eq!(
            Constraint::check_partial(&Xor(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            false
        );
    }

    // Not combinator

    #[test]
    fn partial_not_should_always_be_true() {
        let path = score_of(path_of(vec![1, 2]), 6);
        assert_eq!(
            Constraint::check_partial(&Not(box_of(MaxScore(4))), &path),
            true
        );
        assert_eq!(
            Constraint::check_partial(&Not(box_of(MaxScore(6))), &path),
            true
        );
        assert_eq!(
            Constraint::check_partial(&Not(box_of(MaxScore(12))), &path),
            true
        );
    }

    //
    // Complete constraints
    // (ie once the path is complete - no new vertex will be added)
    //

    // ContainsVertex
    #[test]
    fn complete_contains_vertex_should_ensure_path_has_a_given_vertex() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_complete(&ContainsVertex(VertexId(1)), &path),
            true
        );
        assert_eq!(
            Constraint::check_complete(&ContainsVertex(VertexId(2)), &path),
            true
        );
        assert_eq!(
            Constraint::check_complete(&ContainsVertex(VertexId(3)), &path),
            false
        );
    }

    // ContainsEdge
    #[test]
    fn complete_contains_edge_should_ensure_path_has_a_given_edge() {
        let path = score_of(path_of(vec![1, 2]), 1);
        assert_eq!(
            Constraint::check_complete(&ContainsEdge(Edge(VertexId(1), VertexId(2))), &path),
            true
        );
        assert_eq!(
            Constraint::check_complete(&ContainsEdge(Edge(VertexId(1), VertexId(3))), &path),
            false
        );
        assert_eq!(
            Constraint::check_complete(&ContainsEdge(Edge(VertexId(2), VertexId(3))), &path),
            false
        );
    }

    // OrderedVertices
    #[test]
    fn complete_ordered_vertices_should_detect_invalid_path() {
        let path = score_of(path_of(vec![1, 2]), 1);
        let constraint = OrderedVertices(vec![VertexId(2), VertexId(5), VertexId(1)]);
        assert_eq!(Constraint::check_complete(&constraint, &path), false);
    }

    // Cycle
    #[test]
    fn contains_cycle_should_be_false_on_complete_path_without_cycle() {
        let path = score_of(path_of(vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(Constraint::check_complete(&ContainsCycle, &path), false);
    }

    #[test]
    fn contains_cycle_should_be_true_on_complete_path_with_one_cycle() {
        let path = score_of(path_of(vec![1, 2, 3, 4, 5, 2, 7, 8]), 1);
        assert_eq!(Constraint::check_complete(&ContainsCycle, &path), true);
    }

    #[test]
    fn contains_cycle_should_be_true_on_complete_path_with_more_than_one_cycle() {
        let path = score_of(path_of(vec![1, 2, 3, 4, 5, 2, 7, 8, 3, 9, 10, 5, 11]), 1);
        assert_eq!(Constraint::check_complete(&ContainsCycle, &path), true);
    }

    // MinLength
    #[test]
    fn complete_min_length_should_allow_only_path_with_a_minimum_length() {
        let path = score_of(path_of(vec![1, 2, 3, 4]), 1);
        assert_eq!(path.path.size(), 4);
        assert_eq!(Constraint::check_complete(&MinLength(2), &path), true);
        assert_eq!(Constraint::check_complete(&MinLength(4), &path), true);
        assert_eq!(Constraint::check_complete(&MinLength(5), &path), false);
    }

    // MaxLength
    #[test]
    fn complete_max_length_should_allow_only_path_up_to_max_length() {
        let path = score_of(path_of(vec![1, 2, 3, 4]), 1);
        assert_eq!(path.path.size(), 4);
        assert_eq!(Constraint::check_complete(&MaxLength(2), &path), false);
        assert_eq!(Constraint::check_complete(&MaxLength(4), &path), true);
        assert_eq!(Constraint::check_complete(&MaxLength(5), &path), true);
    }

    // MinScore
    #[test]
    fn complete_min_score_should_allow_only_path_with_a_minimum_score() {
        let path = score_of(path_of(vec![1, 2]), 4);
        assert_eq!(Constraint::check_complete(&MinScore(2), &path), true);
        assert_eq!(Constraint::check_complete(&MinScore(4), &path), true);
        assert_eq!(Constraint::check_complete(&MinScore(5), &path), false);
    }

    // MaxScore
    #[test]
    fn complete_max_score_should_allow_only_path_up_to_max_score() {
        let path = score_of(path_of(vec![1, 2]), 4);
        assert_eq!(Constraint::check_complete(&MaxScore(2), &path), false);
        assert_eq!(Constraint::check_complete(&MaxScore(4), &path), true);
        assert_eq!(Constraint::check_complete(&MaxScore(5), &path), true);
    }

    // Or combinator
    #[test]
    fn complete_or_should_be_true_if_any_of_the_constraints_is_met() {
        let path = score_of(path_of(vec![1, 2]), 13);
        assert_eq!(
            Constraint::check_complete(&Or(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            true
        );
        assert_eq!(
            Constraint::check_complete(&Or(box_of(MaxScore(15)), box_of(MaxScore(12))), &path),
            true
        );
    }

    #[test]
    fn complete_or_should_be_false_if_none_of_the_constraints_can_be_met() {
        let path = score_of(path_of(vec![1, 2]), 20);
        assert_eq!(
            Constraint::check_complete(&Or(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            false
        );
        assert_eq!(
            Constraint::check_complete(&Or(box_of(MaxScore(15)), box_of(MaxScore(12))), &path),
            false
        );
    }

    // And combinator
    #[test]
    fn complete_and_should_be_true_if_both_constraints_are_met() {
        let path = score_of(path_of(vec![1, 2]), 11);
        assert_eq!(
            Constraint::check_complete(&And(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            true
        );
    }

    #[test]
    fn complete_and_should_be_false_if_at_least_one_constraint_is_not_met() {
        let path = score_of(path_of(vec![1, 2]), 13);
        assert_eq!(
            Constraint::check_complete(&And(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            false
        );
    }

    // Xor combinator
    #[test]
    fn complete_xor_should_be_true_if_only_one_out_of_two_constraint_is_met() {
        let path = score_of(path_of(vec![1, 2]), 13);
        assert_eq!(
            Constraint::check_complete(&Xor(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            true
        );
    }

    #[test]
    fn partial_xor_should_be_false_if_both_the_constraints_are_met_or_not_met() {
        let path = score_of(path_of(vec![1, 2]), 16);
        assert_eq!(
            Constraint::check_complete(&Xor(box_of(MaxScore(12)), box_of(MaxScore(15))), &path),
            false
        );
        assert_eq!(
            Constraint::check_complete(&Xor(box_of(MaxScore(20)), box_of(MaxScore(25))), &path),
            false
        );
    }

    // Not combinator

    #[test]
    fn complete_not_should_true_when_wrapped_constraint_is_false() {
        let path = score_of(path_of(vec![1, 2]), 6);
        assert_eq!(
            Constraint::check_complete(&Not(box_of(MaxScore(4))), &path),
            true
        );
        assert_eq!(
            Constraint::check_complete(&Not(box_of(MaxScore(6))), &path),
            false
        );
        assert_eq!(
            Constraint::check_complete(&Not(box_of(MaxScore(12))), &path),
            false
        )
    }

    #[test]
    fn complete_not_should_false_when_wrapped_constraint_is_true() {
        let path = score_of(path_of(vec![1, 2]), 6);
        assert_eq!(
            Constraint::check_complete(&Not(box_of(MaxScore(6))), &path),
            false
        );
        assert_eq!(
            Constraint::check_complete(&Not(box_of(MaxScore(12))), &path),
            false
        );
    }

    // Helper

    fn path_of(vertices: Vec<u64>) -> Path {
        Path {
            vertices: vertices.iter().map(|x| VertexId(*x)).collect(),
        }
    }

    fn score_of(path: Path, score: i64) -> ScoredPath {
        ScoredPath {
            path: path,
            score: score,
        }
    }

    fn box_of<T>(value: T) -> Box<T> {
        Box::new(value)
    }
}
