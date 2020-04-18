use crate::constraint::constraint::Constraint;
use crate::constraint::constraint::Constraint::*;
use crate::graph::Edge;
use crate::graph::VertexId;

use std::collections::HashSet;

/// Best effort to validate the list of constraints provided to make sure they are not incompatible
///
/// Incompatible constraints could be :
/// - Inclusion/Exclusion of the same vertex/edge
/// - Inclusion/Exclusion of cycle
/// - Several Ordered vertex constraints
/// - Min/Max Length/Score appearing several times
/// - Max Length/Score less than a Min Length/Score
/// - ...
/// An incompatible set of constraints should lead to no solution straight away
///
/// Note:
/// This simple validation doesn't look into And/Or/Xor statement, not dig recursively in a bunch of Not(Not(...)) definition
pub fn validate(constraints: &Vec<Constraint>) -> Result<(), String> {
    check_vertex_inclusion_exclusion(constraints)?;
    check_edge_inclusion_exclusion(constraints)?;
    check_vertices_order(constraints)?;
    check_cycle_inclusion_exclusion(constraints)?;
    check_min_max_length(constraints)?;
    check_min_max_score(constraints)?;
    ok()
}

fn check_vertex_inclusion_exclusion(constraints: &Vec<Constraint>) -> Result<(), String> {
    let vertex_inclusions: HashSet<&VertexId> = constraints
        .iter()
        .filter_map(|c| match c {
            ContainsVertex(vid) => Some(vec![vid]),
            ContainsEdge(Edge(src, dst)) => Some(vec![src, dst]),
            _ => None,
        })
        .flatten()
        .collect();
    let vertex_exclusions: HashSet<&VertexId> = constraints
        .iter()
        .filter_map(|nc| match nc {
            Not(c) => match **c {
                ContainsVertex(ref vid) => Some(vec![vid]),
                _ => None,
            },
            _ => None,
        })
        .flatten()
        .collect();

    // Intersection must be empty for the vertices/edge constraint to be compatible
    let intersection: Vec<&&VertexId> =
        vertex_inclusions.intersection(&vertex_exclusions).collect();
    if intersection.is_empty() {
        ok()
    } else {
        fail(format!["Incompatible set of constraints leading to vertices {:?} to be included and excluded at the same time.", intersection])
    }
}

fn check_edge_inclusion_exclusion(constraints: &Vec<Constraint>) -> Result<(), String> {
    let edge_inclusions: HashSet<&Edge> = constraints
        .iter()
        .filter_map(|c| match c {
            ContainsEdge(ref e) => Some(e),
            _ => None,
        })
        .collect();
    let edge_exclusions: HashSet<&Edge> = constraints
        .iter()
        .filter_map(|nc| match nc {
            Not(c) => match **c {
                ContainsEdge(ref e) => Some(e),
                _ => None,
            },
            _ => None,
        })
        .collect();

    // Intersection must be empty for the vertices/edge constraint to be compatible
    let intersection: Vec<&&Edge> = edge_inclusions.intersection(&edge_exclusions).collect();
    if intersection.is_empty() {
        ok()
    } else {
        fail(format!["Incompatible set of constraints leading to edges {:?} to be included and excluded at the same time.", intersection])
    }
}

fn check_vertices_order(_constraints: &Vec<Constraint>) -> Result<(), String> {
    // Nothing to check for now
    ok()
}

fn check_cycle_inclusion_exclusion(constraints: &Vec<Constraint>) -> Result<(), String> {
    let contains_cycle = constraints.iter().find(|c| **c == ContainsCycle).is_some();
    let contains_no_cycle = negated_constraints(constraints)
        .find(|c| **c == ContainsCycle)
        .is_some();
    if contains_cycle != contains_no_cycle {
        ok()
    } else {
        fail(format![
            "Incompatible set of constraints about cycle inclusion and exclusion."
        ])
    }
}

fn check_min_max_length(constraints: &Vec<Constraint>) -> Result<(), String> {
    let min_length: Vec<usize> = constraints
        .iter()
        .filter_map(|c| match c {
            MinLength(l) => Some(*l),
            _ => None,
        })
        .collect();
    let max_length: Vec<usize> = constraints
        .iter()
        .filter_map(|c| match c {
            MaxLength(l) => Some(*l),
            _ => None,
        })
        .collect();

    if min_length.len() > 1 {
        fail(format!["Incompatible set of MinLength constraints: {:?} were defined, having only one is possible", min_length])
    } else if max_length.len() > 1 {
        fail(format!["Incompatible set of MaxLength constraints: {:?} were defined, having only one is possible", max_length])
    } else if !min_length.is_empty() && !max_length.is_empty() {
        match (min_length.first(), max_length.first()) {
            (Some(min), Some(max)) if min > max => fail(format![
                "Incompatible set of min/max length constraints: min={}, max={}",
                min, max
            ]),
            _ => ok(),
        }
    } else {
        ok()
    }
}

fn check_min_max_score(constraints: &Vec<Constraint>) -> Result<(), String> {
    let min_score: Vec<i64> = constraints
        .iter()
        .filter_map(|c| match c {
            MinScore(l) => Some(*l),
            _ => None,
        })
        .collect();
    let max_score: Vec<i64> = constraints
        .iter()
        .filter_map(|c| match c {
            MaxScore(l) => Some(*l),
            _ => None,
        })
        .collect();

    if min_score.len() > 1 {
        fail(format!["Incompatible set of MinScore constraints: {:?} were defined, having only one is possible", min_score])
    } else if max_score.len() > 1 {
        fail(format!["Incompatible set of MaxScore constraints: {:?} were defined, having only one is possible", max_score])
    } else if !min_score.is_empty() && !max_score.is_empty() {
        match (min_score.first(), max_score.first()) {
            (Some(min), Some(max)) if min > max => fail(format![
                "Incompatible set of min/max score constraints: min={}, max={}",
                min, max
            ]),
            _ => ok(),
        }
    } else {
        ok()
    }
}

// Helpers

fn ok<T>() -> Result<(), T> {
    Ok(())
}

fn fail<T>(failure: T) -> Result<(), T> {
    Err(failure)
}

// Extract negated constraints
fn negated_constraints<'a>(
    constraints: impl IntoIterator<Item = &'a Constraint>,
) -> impl Iterator<Item = &'a Constraint> {
    constraints.into_iter().filter_map(|nc| match nc {
        Not(c) => Some(&**c),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_vertex_inclusion_exclusion_should_accept_when_vertices_inclusion_and_exclusion_are_compatible(
    ) {
        let constraints = vec![
            ContainsVertex(vertex(4)),
            ContainsVertex(vertex(5)),
            ContainsVertex(vertex(6)),
            ContainsEdge(edge(5, 4)),
            Not(Box::new(ContainsVertex(vertex(3)))),
        ];
        assert_eq!(check_vertex_inclusion_exclusion(&constraints), Ok(()))
    }

    #[test]
    fn check_vertex_inclusion_exclusion_should_reject_when_a_vertex_is_included_and_excluded_at_the_same_time_1(
    ) {
        let constraints = vec![
            ContainsVertex(vertex(3)),
            Not(Box::new(ContainsVertex(vertex(3)))),
        ];
        assert_eq!(
            check_vertex_inclusion_exclusion(&constraints),
            Err("Incompatible set of constraints leading to vertices [VertexId(3)] to be included and excluded at the same time.".to_string())
        )
    }

    // Max / Min Length

    #[test]
    fn check_min_max_length_should_accept_compatible_min_and_max_length_constraint() {
        let constraints = vec![MaxLength(6), MinLength(5)];
        assert_eq!(check_min_max_length(&constraints), Ok(()))
    }

    #[test]
    fn check_min_max_length_should_reject_incompatible_min_length_constraint() {
        let constraints = vec![MinLength(5), MinLength(6)];
        assert_eq!(
            check_min_max_length(&constraints),
            Err("Incompatible set of MinLength constraints: [5, 6] were defined, having only one is possible".to_string())
        )
    }

    #[test]
    fn check_min_max_length_should_reject_incompatible_max_length_constraint() {
        let constraints = vec![MaxLength(5), MaxLength(6)];
        assert_eq!(
            check_min_max_length(&constraints),
            Err("Incompatible set of MaxLength constraints: [5, 6] were defined, having only one is possible".to_string())
        )
    }

    #[test]
    fn check_min_max_length_should_reject_incompatible_min_and_max_length_constraint() {
        let constraints = vec![MaxLength(5), MinLength(6)];
        assert_eq!(
            check_min_max_length(&constraints),
            Err("Incompatible set of min/max length constraints: min=6, max=5".to_string())
        )
    }

    // Max / Min Score

    #[test]
    fn check_min_max_length_should_accept_compatible_min_and_max_score_constraint() {
        let constraints = vec![MaxScore(6), MinScore(5)];
        assert_eq!(check_min_max_score(&constraints), Ok(()))
    }

    #[test]
    fn check_min_max_length_should_reject_incompatible_min_score_constraint() {
        let constraints = vec![MinScore(5), MinScore(6)];
        assert_eq!(
            check_min_max_score(&constraints),
            Err("Incompatible set of MinScore constraints: [5, 6] were defined, having only one is possible".to_string())
        )
    }

    #[test]
    fn check_min_max_length_should_reject_incompatible_max_score_constraint() {
        let constraints = vec![MaxScore(5), MaxScore(6)];
        assert_eq!(
            check_min_max_score(&constraints),
            Err("Incompatible set of MaxScore constraints: [5, 6] were defined, having only one is possible".to_string())
        )
    }

    #[test]
    fn check_min_max_length_should_reject_incompatible_min_and_max_score_constraint() {
        let constraints = vec![MaxScore(5), MinScore(6)];
        assert_eq!(
            check_min_max_score(&constraints),
            Err("Incompatible set of min/max score constraints: min=6, max=5".to_string())
        )
    }

    // Helpers
    fn vertex(v: u64) -> VertexId {
        VertexId(v)
    }

    fn edge(src: u64, end: u64) -> Edge {
        Edge(vertex(src), vertex(end))
    }
}
