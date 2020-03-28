use crate::directed_graph::DirectedGraph;
use crate::format::utils;
use crate::graph::Edge;
use crate::graph::VertexId;
use crate::graph_command::GraphCommand;
use crate::graph_command::GraphCommand::AddEdge;
use crate::graph_command::GraphCommand::AddVertex;

use lazy_static::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufWriter, Write};

///
/// Reading TGF files
///

/// Reads a TGF into a DirectedGraph
pub fn read(file: File) -> Result<DirectedGraph, String> {
    utils::read(file, parse_line, is_comment)
}

/// Reads a TGF file as a list of commands
pub fn read_as_commands(file: File) -> Result<Vec<GraphCommand>, String> {
    utils::read_as_commands(file, parse_line, is_comment)
}

// Parses a line into a GraphCommand
fn parse_line(line: &str) -> Result<GraphCommand, String> {
    lazy_static! {
        static ref VERTEX_RE: Regex = Regex::new(r"^(\d+)(.*)$").unwrap();
        static ref EDGE_RE: Regex = Regex::new(r"^(\d+)\s+(\d+)(.*)$").unwrap();
    }

    if let Some(cap) = EDGE_RE.captures_iter(line).next() {
        let v1 = &cap[1].parse::<u64>().unwrap();
        let v2 = &cap[2].parse::<u64>().unwrap();
        Ok(AddEdge(VertexId(*v1), VertexId(*v2)))
    } else if let Some(cap) = VERTEX_RE.captures_iter(line).next() {
        let v1 = &cap[1].parse::<u64>().unwrap();
        Ok(AddVertex(VertexId(*v1)))
    } else {
        Err(format!["Couldn't parse '{}'", line])
    }
}

fn is_comment(line: &str) -> bool {
    line.trim().starts_with("#")
}

///
/// Writing TGF files
///

/// Saves a DirectedGraph into a TGF
pub fn save(graph: &DirectedGraph, filename: String) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut buffered = BufWriter::new(file);
    for vertex in graph.vertices() {
        let VertexId(vertex_id) = vertex;
        writeln!(buffered, "{}", vertex_id)?;
    }
    for edge in graph.edges() {
        let Edge(VertexId(src), VertexId(dest)) = edge;
        writeln!(buffered, "{} {}", src, dest)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vertex_line_with_label_should_work() {
        assert_eq![
            parse_line("123456 This label will be ignored"),
            Ok(AddVertex(VertexId(123456)))
        ]
    }

    #[test]
    fn parse_edge_line_with_label_should_work() {
        assert_eq![
            parse_line("123456 784695 This label will be ignored"),
            Ok(AddEdge(VertexId(123456), VertexId(784695)))
        ]
    }

    #[test]
    fn parse_vertex_line_without_label_should_work() {
        assert_eq![parse_line("123456"), Ok(AddVertex(VertexId(123456)))]
    }

    #[test]
    fn parse_edge_line_without_label_should_work() {
        assert_eq![
            parse_line("123456 784695 This label will be ignored"),
            Ok(AddEdge(VertexId(123456), VertexId(784695)))
        ]
    }

    #[test]
    fn parse_ill_formatted_line_should_fail() {
        assert_eq![
            parse_line("a123456 784695 Label"),
            Err("Couldn't parse 'a123456 784695 Label'".to_string())
        ]
    }
}
