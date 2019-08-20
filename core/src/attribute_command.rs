use crate::attribute_repo::*;
use crate::graph::*;

pub enum AttributeCommand {
    AddVertexAttr(VertexId, AttrId),
    RemoveVertexAttr(VertexId, AttrId),
    AddEdgeAttr(VertexId, VertexId, AttrId),
    RemoveEdgeAttr(VertexId, VertexId, AttrId),
}

impl AttributeCommand {
    pub fn revert(command: AttributeCommand) -> AttributeCommand {
        use AttributeCommand::*;
        match command {
            AddVertexAttr(v, a) => RemoveVertexAttr(v, a),
            RemoveVertexAttr(v, a) => AddVertexAttr(v, a),
            AddEdgeAttr(v1, v2, a) => RemoveEdgeAttr(v1, v2, a),
            RemoveEdgeAttr(v1, v2, a) => AddEdgeAttr(v1, v2, a),
        }
    }

    pub fn apply_commands<T>(commands: Vec<AttributeCommand>, repo: &mut AttrRepo<T>) -> () {
        for command in commands.iter() {
            command.apply_to(repo);
        }
    }

    pub fn apply_to<T>(&self, repo: &mut AttrRepo<T>) {
        use AttributeCommand::*;
        match self {
            AddVertexAttr(v, a) => unimplemented!(),
            RemoveVertexAttr(v, a) => unimplemented!(),
            AddEdgeAttr(v1, v2, a) => unimplemented!(),
            RemoveEdgeAttr(v1, v2, a) => unimplemented!(),
        }
    }
}
