use crate::token::Token;

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum NodeType {
    Binary,
    Literal,
}

#[repr(C)]
#[derive(Debug)]
pub struct Node<'a> {
    pub node_type: NodeType,
    pub token: &'a Token,
    pub num_sub_nodes: u32,
}

pub struct NodeCtx {
    node_id: usize,
    num_nodes: usize,
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub nodes: Vec<Node<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new(tokens: Vec<&'a Token>) -> Self {
        let nodes = Vec::with_capacity(tokens.len());
        Self { nodes }
    }

    pub fn start_node(&mut self, token: &'a Token, node_type: NodeType) -> NodeCtx {
        let node_id = self.nodes.len();
        let new_node = Node {
            node_type,
            token,
            num_sub_nodes: 0,
        };
        self.nodes.push(new_node);
        
        NodeCtx {
            node_id,
            num_nodes: self.nodes.len(),
        }
    }

    pub fn end_node(&mut self, node_ctx: NodeCtx) {
        self.nodes.get_mut(node_ctx.node_id).unwrap().num_sub_nodes =
            (self.nodes.len() - node_ctx.num_nodes) as u32
    }

    pub fn dump(&self, node_id: usize) -> String {
        let node = &self.nodes[node_id];
        let node_type = node.node_type;

        match node_type {
            NodeType::Binary => {
                // recursion case
                let lhs_id = node_id + 1;
                let rhs_id = lhs_id + self.nodes[lhs_id].num_sub_nodes as usize;

                format!(
                    "binary({:?}, {}, {})",
                    node.token.kind,
                    self.dump(lhs_id),
                    self.dump(rhs_id)
                )
            }
            NodeType::Literal => format!("literal({:?})", node.token), // base case
        }
    }
}
