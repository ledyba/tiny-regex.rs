#[derive(Clone)]
pub enum Node {
  Literal(String),
  Or(Vec<Node>),
  Concat(Vec<Node>),
  Repeat(Box<Node>),
}

pub fn literal(s: &str) -> Node {
  Node::Literal(s.to_string())
}

pub fn or(nodes: &[Node]) -> Node {
  Node::Or(nodes.to_vec())
}

pub fn concat(nodes: &[Node]) -> Node {
  Node::Concat(nodes.to_vec())
}

pub fn repeat(node: Node) -> Node {
  Node::Repeat(Box::new(node))
}
