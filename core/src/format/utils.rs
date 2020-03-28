//! Module providing helper to read/write to graph files

use crate::directed_graph::DirectedGraph;
use crate::graph_command::GraphCommand;
use std::fs::File;
use std::io::{BufRead, BufReader};

///
/// Reading Graph files
///

/// Reads a file into a DirectedGraph
pub fn read<ParseFn, IsCommentFn>(
    file: File,
    parse_line: ParseFn,
    is_comment: IsCommentFn,
) -> Result<DirectedGraph, String>
where
    ParseFn: Fn(&str) -> Result<GraphCommand, String>,
    IsCommentFn: Fn(&str) -> bool,
{
    read_as_commands(file, parse_line, is_comment).map(|commands| {
        let mut graph = DirectedGraph::new();
        GraphCommand::apply_commands(commands, &mut graph);
        graph
    })
}

/// Reads a TGF file as a list of commands
pub fn read_as_commands<ParseFn, IsCommentFn>(
    file: File,
    parse_line: ParseFn,
    is_comment: IsCommentFn,
) -> Result<Vec<GraphCommand>, String>
where
    ParseFn: Fn(&str) -> Result<GraphCommand, String>,
    IsCommentFn: Fn(&str) -> bool,
{
    let reader = BufReader::new(file);
    let mut result: Vec<GraphCommand> = vec![];
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if !line.is_empty() && !is_comment(&line) {
            match parse_line(&line) {
                Ok(command) => result.push(command),
                Err(msg) => return Err(format!["Error at line {}: {}", index + 1, msg]),
            }
        }
    }
    Ok(result)
}
