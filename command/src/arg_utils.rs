use gc_core::constraint::constraint::Constraint::*;
use gc_core::constraint::constraint::*;
use gc_core::graph::Edge;
use gc_core::graph::VertexId;

// Parsing arguments

/// Groups a list of vertices 2 by 2 to produce edges
pub fn as_vertex_tuple(vids: Vec<VertexId>) -> Option<Vec<(VertexId, VertexId)>> {
    if vids.len() % 2 == 0 {
        let mut res = vec![];
        for i in (0..vids.len()).step_by(2) {
            res.push((vids[i], vids[i + 1]));
        }
        Some(res)
    } else {
        None
    }
}

pub fn parse_vertex_id(v: &str) -> Option<u64> {
    v.parse::<u64>().ok()
}

pub fn parse_vertex_id_list(ids: Vec<&str>) -> Option<Vec<VertexId>> {
    let mut res = vec![];
    for id in ids {
        match parse_vertex_id(id) {
            Some(id) => res.push(VertexId(id)),
            None => return None,
        };
    }
    Some(res)
}

pub fn parse_edge_list(ids: Vec<&str>) -> Option<Vec<Edge>> {
    parse_vertex_id_list(ids)
        .and_then(|ids| as_vertex_tuple(ids))
        .map(|pairs| pairs.iter().map(|(src, dst)| Edge(*src, *dst)).collect())
}

pub fn confirmation_yes_no(msg: &str) -> bool {
    let mut buffer = String::new();
    println!("{}", msg);
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid UTF-8 bytes");
    match buffer.to_string().trim().as_ref() {
        "yes" | "y" => true,
        "no" | _ => false,
    }
}

// Building constraints

pub fn build_constraint_include(ids: Vec<VertexId>) -> Vec<Constraint> {
    ids.iter().map(|c| ContainsVertex(*c)).collect()
}

pub fn build_constraint_exclude(ids: Vec<VertexId>) -> Vec<Constraint> {
    ids.iter()
        .map(|c| Not(Box::new(ContainsVertex(*c))))
        .collect()
}

pub fn build_constraint_include_edges(edges: Vec<Edge>) -> Vec<Constraint> {
    edges.iter().map(|e| ContainsEdge(*e)).collect()
}

pub fn build_constraint_exclude_edges(edges: Vec<Edge>) -> Vec<Constraint> {
    edges
        .iter()
        .map(|e| Not(Box::new(ContainsEdge(*e))))
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

pub fn option_of<T, Thunk>(flag: bool, thunk: Thunk) -> Option<T>
where
    Thunk: FnOnce() -> T,
{
    match flag {
        false => None,
        true => Some(thunk()),
    }
}
