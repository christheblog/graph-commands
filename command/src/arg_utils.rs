


use hg_core::constraint::constraint::Constraint::*;
use hg_core::constraint::constraint::*;
use hg_core::graph::VertexId;
use hg_core::graph::Edge;

// Building constraints

pub fn build_constraint_include(ids: Vec<VertexId>) -> Vec<Constraint> {
    ids.iter().map(|c| ContainsVertex(*c)).collect()
}

pub fn build_constraint_exclude(ids: Vec<VertexId>) -> Vec<Constraint> {
    ids.iter()
        .map(|c| Not(Box::new(ContainsVertex(*c))))
        .collect()
}

pub fn build_constraint_ordered(ids: Vec<VertexId>) -> Constraint {
    OrderedVertices(ids)
}

pub fn build_constraint_include_cycle() -> Constraint {
    ContainsCycle
}

pub fn build_constraint_no_cycle() -> Constraint {
    Not(Box::new(ContainsCycle))
}

pub fn build_constraint_min_length(len: usize) -> Constraint {
    MinLength(len)
}

pub fn build_constraint_max_length(len: usize) -> Constraint {
    MaxLength(len)
}

pub fn build_constraint_exact_length(len: usize) -> Vec<Constraint> {
    vec![MinLength(len), MaxLength(len)]
}

pub fn build_constraint_min_score(score: i64) -> Constraint {
    MinScore(score)
}

pub fn build_constraint_max_score(score: i64) -> Constraint {
    MinScore(score)
}

pub fn build_constraint_exact_score(score: i64) -> Vec<Constraint> {
    vec![MinScore(score), MaxScore(score)]
}

// Helpers

pub fn option_of<T,Thunk>(flag: bool, thunk: Thunk) -> Option<T>
where Thunk: FnOnce() -> T {
    match flag {
        false => None,
        true => Some(thunk())
    }
}
