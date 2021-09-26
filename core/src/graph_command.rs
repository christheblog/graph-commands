//! Commands to iteratively build/update a graph by adding/removing vertices and edges
use crate::directed_graph::DirectedGraph;
use crate::graph::{Edge, VertexId};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GraphCommand {
    AddVertex(VertexId),
    RemoveVertex(VertexId),
    AddEdge(VertexId, VertexId),
    RemoveEdge(VertexId, VertexId),
}

impl GraphCommand {
    pub fn revert(command: GraphCommand) -> GraphCommand {
        use GraphCommand::*;
        match command {
            AddVertex(v) => RemoveVertex(v),
            RemoveVertex(v) => AddVertex(v),
            AddEdge(v1, v2) => RemoveEdge(v1, v2),
            RemoveEdge(v1, v2) => AddEdge(v1, v2),
        }
    }

    pub fn apply_commands(commands: Vec<GraphCommand>, graph: &mut DirectedGraph) -> () {
        for command in commands.iter() {
            command.apply_to(graph);
        }
    }

    pub fn apply_to(&self, graph: &mut DirectedGraph) {
        use GraphCommand::*;
        match self {
            AddVertex(v) => {
                graph.add_vertex(*v);
            }
            RemoveVertex(v) => {
                graph.remove_vertex(*v);
            }
            AddEdge(v1, v2) => {
                graph.add_edge(Edge(*v1, *v2));
            }
            RemoveEdge(v1, v2) => {
                graph.remove_edge(Edge(*v1, *v2));
            }
        }
    }

    pub fn as_commands(graph: &DirectedGraph) -> Vec<GraphCommand> {
        use GraphCommand::*;
        let mut res: Vec<GraphCommand> = vec![];
        for &vertex_id in graph.vertices() {
            res.push(AddVertex(vertex_id))
        }
        for &Edge(v1, v2) in graph.edges() {
            res.push(AddEdge(v1, v2))
        }
        res
    }
}
