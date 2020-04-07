//! Collection of useful functions for command-line tools

use hg_core::graph::VertexId;
use hg_core::graph_command::GraphCommand;
use hg_core::graph_command::GraphCommand::AddEdge;
use hg_core::graph_command::GraphCommand::AddVertex;
use hg_core::graph_command::GraphCommand::RemoveEdge;
use hg_core::graph_command::GraphCommand::RemoveVertex;
use std::error::Error;
use std::fs;
use std::io;
use std::path;

use hg_core::directed_graph::DirectedGraph;
use hg_core::format::gcmd;

pub const GRAPH_ROOT_DIR: &str = ".graph";
pub const COMMANDS_FILE: &str = "commands";
pub const LOCK_FILE: &str = "lock";

/// Init the directories / files necessary to have a working empty graph
pub fn init(root_dir: &str) -> io::Result<()> {
    touch(command_path(&root_dir).as_ref())
}

/// Creates a lock file indicating a command is already getting process
/// Any command execution be put on hold / fail straight away if the lock file exists
pub fn lock(root_dir: &str) -> io::Result<()> {
    touch(lock_path(root_dir).as_ref())
}

/// Removes the lock file to indicate
pub fn unlock(root_dir: &str) -> io::Result<()> {
    fs::remove_file(lock_path(root_dir).as_ref())
}

/// Test if the current graph is locked
pub fn is_locked(root_dir: &str) -> bool {
    lock_path(root_dir).as_ref().exists()
}

pub fn with_lock<T, Block>(root_dir: &str, block: Block) -> io::Result<T>
where
    Block: FnOnce() -> io::Result<T>,
{
    lock(root_dir)?;
    let res = block();
    unlock(root_dir)?;
    res
}

/// Loads the graph into memory
pub fn load_graph(root_dir: &str) -> Result<DirectedGraph, String> {
    load_graph_from_path(command_path(root_dir).as_ref())
}

fn load_graph_from_path(filepath: &path::Path) -> Result<DirectedGraph, String> {
    match fs::File::open(filepath) {
        Ok(commands) => Ok(gcmd::read(commands)?),
        Err(io_err) => Err(io_err.description().to_string()),
    }
}

pub fn save_graph_as_commands(filepath: &str, graph: &DirectedGraph) -> io::Result<()> {
    let command_path = command_path(filepath);
    gcmd::save(graph, command_path.as_ref().to_str().expect("Invalid path. (UTF-9 ?)"))
}

/// Cleans-up the graph directory structure
/// This can not be undone
pub fn clean(root_dir: &str) -> std::io::Result<()> {
    with_lock(root_dir, || {
        std::fs::remove_dir_all(root_path(root_dir).as_ref())
    })
}

/// Adds a Vertex to the graph
pub fn add_vertex(root_dir: &str, vid: VertexId) -> std::io::Result<()> {
    apply_graph_commands(root_dir, vec![AddVertex(vid)])
}

/// Adds a list of vertices to a graph
pub fn add_vertices(root_dir: &str, vids: Vec<VertexId>) -> std::io::Result<()> {
    apply_graph_commands(root_dir, vids.iter().map(|vid| AddVertex(*vid)).collect())
}

/// Adds an edge to the graph
pub fn add_edge(root_dir: &str, src: VertexId, dst: VertexId) -> std::io::Result<()> {
    apply_graph_commands(root_dir, vec![AddEdge(src, dst)])
}

/// Adds an edge to the graph
pub fn add_edges(root_dir: &str, edges: Vec<(VertexId, VertexId)>) -> std::io::Result<()> {
    apply_graph_commands(
        root_dir,
        edges
            .iter()
            .map(|(src, dest)| AddEdge(*src, *dest))
            .collect(),
    )
}

/// Removes a Vertex from the graph
pub fn remove_vertex(root_dir: &str, vid: VertexId) -> std::io::Result<()> {
    apply_graph_commands(root_dir, vec![RemoveVertex(vid)])
}

/// Removes a list of vertices from the graph
pub fn remove_vertices(root_dir: &str, vids: Vec<VertexId>) -> std::io::Result<()> {
    apply_graph_commands(
        root_dir,
        vids.iter().map(|vid| RemoveVertex(*vid)).collect(),
    )
}

/// Removes an edge to the graph
pub fn remove_edges(root_dir: &str, edges: Vec<(VertexId, VertexId)>) -> std::io::Result<()> {
    apply_graph_commands(
        root_dir,
        edges
            .iter()
            .map(|(src, dest)| RemoveEdge(*src, *dest))
            .collect(),
    )
}

/// Groups a list of vertices 2 by 2 to produce edges
pub fn as_vertex_tuple(vids: Vec<VertexId>) -> Option<Vec<(VertexId, VertexId)>> {
    if vids.len() % 2 == 0 {
        let mut res = vec![];
        for i in (0..vids.len()).step_by(2) {
            res.push((vids[i], vids[i+1]));
        }
        Some(res)
    } else {
        None
    }
}

pub fn parse_vertex_id(v: &str) -> Option<u64> {
    v.parse::<u64>().ok()
}

pub fn confirmation_yes_no(msg: &str) -> bool {
    let mut buffer = String::new();
    println!("{}", msg);
    std::io::stdin().read_line(&mut buffer)
        .expect("Invalid UTF-8 bytes");
    match buffer.to_string().trim().as_ref() {
        "yes" | "y" => true,
        "no" | _ => false,
    }
}

/// Helpers

fn touch(path: &path::Path) -> io::Result<()> {
    // Making sure parent directory exists
    path.parent().map(|parent| fs::create_dir_all(parent));
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .map(|_| ())
}

fn lock_path(root_dir: &str) -> Box<path::Path> {
    path::Path::new(root_dir).join(LOCK_FILE).into_boxed_path()
}

fn root_path(root_dir: &str) -> Box<path::Path> {
    path::Path::new(root_dir)
        .join(GRAPH_ROOT_DIR)
        .into_boxed_path()
}

fn command_path(root_dir: &str) -> Box<path::Path> {
    path::Path::new(root_dir)
        .join(GRAPH_ROOT_DIR)
        .join(COMMANDS_FILE)
        .into_boxed_path()
}

// Applies a GraphCommand on the file, making sure the lock is acquired and released
fn apply_graph_commands(root_dir: &str, commands: Vec<GraphCommand>) -> std::io::Result<()> {
    with_lock(root_dir, || {
        let path = command_path(root_dir);
        gcmd::add_commands(path.as_ref().to_str().unwrap(), commands)
    })
}
