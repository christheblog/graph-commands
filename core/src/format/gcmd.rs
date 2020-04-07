//! 'gcmd' file format is a just list ofgraph  command that need to be applied in order to recreate a graph
//!
//! It can contain any number of Vertices / Edges additions and deletions
//!
//! A command is one of :
//! - AddVertex <id>
//! - AddEdge <id> <id>
//! - RemoveVertex <id>
//! - RemoveEdge <id>
//!
//! One command per line. A Commented line starts with #

use crate::directed_graph::DirectedGraph;
use crate::format::utils;
use crate::graph::Edge;
use crate::graph::VertexId;
use crate::graph_command::GraphCommand;
use crate::graph_command::GraphCommand::AddEdge;
use crate::graph_command::GraphCommand::AddVertex;
use crate::graph_command::GraphCommand::RemoveEdge;
use crate::graph_command::GraphCommand::RemoveVertex;

use lazy_static::*;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

///
/// Reading a Command file
///

/// Reads a command file directly into a DirectedGraph
pub fn read(file: File) -> Result<DirectedGraph, String> {
    utils::read(file, parse_line, is_comment)
}

/// Reads a command file into a list of ordered commands
pub fn read_as_commands(file: File) -> Result<Vec<GraphCommand>, String> {
    utils::read_as_commands(file, parse_line, is_comment)
}

// Parses a line into a GraphCommand
fn parse_line(line: &str) -> Result<GraphCommand, String> {
    lazy_static! {
        static ref ADD_VERTEX_RE: Regex = Regex::new(r"^AddVertex (\d+)$").unwrap();
        static ref ADD_EDGE_RE: Regex = Regex::new(r"^AddEdge (\d+)\s+(\d+)$").unwrap();
        static ref REMOVE_VERTEX_RE: Regex = Regex::new(r"^RemoveVertex (\d+)$").unwrap();
        static ref REMOVE_EDGE_RE: Regex = Regex::new(r"^RemoveEdge (\d+)\s+(\d+)$").unwrap();
    }

    if let Some(cap) = ADD_EDGE_RE.captures_iter(line).next() {
        let v1 = &cap[1].parse::<u64>().unwrap();
        let v2 = &cap[2].parse::<u64>().unwrap();
        Ok(AddEdge(VertexId(*v1), VertexId(*v2)))
    } else if let Some(cap) = ADD_VERTEX_RE.captures_iter(line).next() {
        let v1 = &cap[1].parse::<u64>().unwrap();
        Ok(AddVertex(VertexId(*v1)))
    } else if let Some(cap) = REMOVE_EDGE_RE.captures_iter(line).next() {
        let v1 = &cap[1].parse::<u64>().unwrap();
        let v2 = &cap[2].parse::<u64>().unwrap();
        Ok(RemoveEdge(VertexId(*v1), VertexId(*v2)))
    } else if let Some(cap) = REMOVE_VERTEX_RE.captures_iter(line).next() {
        let v1 = &cap[1].parse::<u64>().unwrap();
        Ok(RemoveVertex(VertexId(*v1)))
    } else {
        Err(format!["Couldn't parse '{}'", line])
    }
}

fn is_comment(line: &str) -> bool {
    line.trim().starts_with("#")
}

///
/// Writing gcmd files
///

/// Saves a DirectedGraph into a gcmd file
pub fn save(graph: &DirectedGraph, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut buffered = BufWriter::new(file);
    for vertex in graph.vertices() {
        let VertexId(vertex_id) = vertex;
        writeln![buffered, "AddVertex {}", vertex_id]?;
    }
    for edge in graph.edges() {
        let Edge(VertexId(src), VertexId(dest)) = edge;
        writeln![buffered, "AddEdge {} {}", src, dest]?;
    }
    Ok(())
}

/// Updates an existing file with a list of GraphCommand
/// New commands will be appended at the end of the file
pub fn add_commands(filename: &str, commands: Vec<GraphCommand>) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).append(true).open(filename)?;
    let mut buffered = BufWriter::new(file);
    for command in commands {
        let line = command_into_line(command);
        writeln![buffered, "{}", line]?;
    }
    Ok(())
}

fn command_into_line(command: GraphCommand) -> String {
    use GraphCommand::*;
    match command {
        AddVertex(VertexId(vid)) => format!["AddVertex {}", vid],
        RemoveVertex(VertexId(vid)) => format!["RemoveVertex {}", vid],
        AddEdge(VertexId(src), VertexId(dest)) => format!["AddEdge {} {}", src, dest],
        RemoveEdge(VertexId(src), VertexId(dest)) => format!["RemoveEdge {} {}", src, dest],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_vertex_line_should_work() {
        assert_eq![
            parse_line("AddVertex 123456"),
            Ok(AddVertex(VertexId(123456)))
        ]
    }

    #[test]
    fn parse_add_edge_line_should_work() {
        assert_eq![
            parse_line("AddEdge 123456 784695"),
            Ok(AddEdge(VertexId(123456), VertexId(784695)))
        ]
    }

    #[test]
    fn parse_remove_vertex_line_should_work() {
        assert_eq![
            parse_line("RemoveVertex 123456"),
            Ok(RemoveVertex(VertexId(123456)))
        ]
    }

    #[test]
    fn parse_remove_edge_line_should_work() {
        assert_eq![
            parse_line("RemoveEdge 123456 784695"),
            Ok(RemoveEdge(VertexId(123456), VertexId(784695)))
        ]
    }

    #[test]
    fn parse_ill_formatted_line_should_fail() {
        assert_eq![
            parse_line("AddEdge a123456 784695"),
            Err("Couldn't parse 'AddEdge a123456 784695'".to_string())
        ]
    }

    #[test]
    fn command_into_line_translate_add_vertex() {
        assert_eq![
            command_into_line(AddVertex(VertexId(123456))),
            "AddVertex 123456"
        ]
    }

    #[test]
    fn command_into_line_translate_remove_vertex() {
        assert_eq![
            command_into_line(RemoveVertex(VertexId(123456))),
            "RemoveVertex 123456"
        ]
    }

    #[test]
    fn command_into_line_translate_add_edge() {
        assert_eq![
            command_into_line(AddEdge(VertexId(123456), VertexId(784695))),
            "AddEdge 123456 784695"
        ]
    }

    #[test]
    fn command_into_line_translate_remove_edge() {
        assert_eq![
            command_into_line(RemoveEdge(VertexId(123456), VertexId(784695))),
            "RemoveEdge 123456 784695"
        ]
    }
}
