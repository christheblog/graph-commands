use crate::graph::*;
use crate::attribute::mapping::*;

pub enum AttributeCommand<T> {
    AddVertexAttr(VertexId, T),
    RemoveVertexAttr(VertexId),
    AddEdgeAttr(VertexId, VertexId, T),
    RemoveEdgeAttr(VertexId, VertexId),
}

impl<T> AttributeCommand<T> {

    // Vertex attribute mapping

    pub fn apply_vertex_command_to<V>(command: AttributeCommand<V>, mapping: &mut VertexAttrMapping<V>) -> bool {
        use AttributeCommand::*;
        match command {
            AddVertexAttr(v, value) => mapping.add(v, value),
            RemoveVertexAttr(v) => mapping.remove(&v),
            _ => false
        }
    }

    pub fn apply_vertex_commands_to<V>(commands: Vec<AttributeCommand<V>>, mapping: &mut VertexAttrMapping<V>) -> () {
        for c in commands {
            AttributeCommand::<V>::apply_vertex_command_to(c, mapping);
        }
    }

    // Edge attribute mapping

    pub fn apply_edge_command_to<V>(command: AttributeCommand<V>, mapping: &mut EdgeAttrMapping<V>) -> bool {
        use AttributeCommand::*;
        match command {
            AddEdgeAttr(v1, v2, value) => mapping.add(Edge(v1,v2), value),
            RemoveEdgeAttr(v1, v2) => mapping.remove(&Edge(v1,v2)),
            _ => false
        }
    }

    pub fn apply_edge_commands_to<V>(commands: Vec<AttributeCommand<V>>, mapping: &mut EdgeAttrMapping<V>) -> () {
        for c in commands {
            AttributeCommand::<V>::apply_edge_command_to(c, mapping);
        }
    }
}
