use crate::ast::Node;
pub enum OpCode {
  Consume(char),
}

pub fn test(codes: &Vec<OpCode>, target: &str) -> bool {
  let mut pc: usize = 0;
  let end = codes.len();
  let mut target = target;
  loop {
    if pc == end {
      return target.len() == 0;
    }
    match &codes[pc] {
      OpCode::Consume(ch) => {

      }
    }
  }
}

pub fn compile(node: &Node) -> Vec<OpCode> {
  match node {
    Node::Literal(literal) => {
      literal
        .chars()
        .map(|c| OpCode::Consume(c))
        .collect()
    }
    Node::Concat(nodes) => {
      nodes
        .iter()
        .map(|node| compile(node))
        .flatten()
        .collect()
    }
    _ => {
      unimplemented!();
    }
  }
}