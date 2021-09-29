use crate::ast::Node;

fn is_prefix_of(prefix: &str, target: &str) -> bool{
  if prefix.len() > target.len() {
    return false;
  }
  return prefix == &target[..prefix.len()];
}

pub fn match_prefix<'a>(node: &Node, target: &'a str) -> Result<&'a str, &'a str> {
  match node {
    Node::Literal(prefix) => {
      if  is_prefix_of(prefix, target) {
        Ok(&target[prefix.len()..])
      } else {
        Err(target)
      }
    },
    Node::Or(nodes) => {
      for node in nodes.iter() {
        match match_prefix(node, target) {
          Ok(left) => {
            return Ok(left);
          }
          _ => {}
        }
      }
      Err(target)
    },
    Node::Concat(nodes) => {
      let mut left = target;
      for node in nodes.iter() {
        match match_prefix(node, left) {
          Ok(new_left) => {
            left = new_left;
          }
          Err(_) => {
            return Err(target);
          }
        }
      }
      return Ok(left);
    }
    Node::Repeat(node) => {
      let mut left = target;
      while let Ok(new_left) = match_prefix(node, left) {
        left = new_left;
      }
      Ok(left)
    }
  }
}

pub fn test(node: &Node, target: &str) -> bool {
  match match_prefix(node, target) {
    Ok("") => true,
    _ => false,
  }
}

#[cfg(test)]
mod test {
  use super::{is_prefix_of, test};
  use crate::ast;

  #[test]
  fn prefix_test() {
    assert!(is_prefix_of("s_", "s_str"));
    assert!(is_prefix_of("s_", "s_"));
    assert!(!is_prefix_of("s_", "n_str"));
    assert!(!is_prefix_of("s_", "s"));
  }

  #[test]
  fn literal_test() {
    assert!(test(&ast::literal("test"), "test"));
    assert!(!test(&ast::literal("test"), ""));
    assert!(!test(&ast::literal("test"), "test1"));
    assert!(!test(&ast::literal("test"), "tes"));
  }
  #[test]
  fn or_test() {
    let node = &ast::or(&[ast::literal("a"), ast::literal("b")]);
    assert!(test(node, "a"));
    assert!(test(node, "b"));
    assert!(!test(node, "c"));
    assert!(!test(node, "aa"));
    assert!(!test(node, "bb"));
    assert!(!test(node, "ab"));
  }

  #[test]
  fn concat_test() {
    let node = &ast::concat(&[ast::literal("a"), ast::literal("b")]);
    assert!(test(node, "ab"));
    assert!(!test(node, "abb"));
    assert!(!test(node, "aab"));
    assert!(!test(node, "a"));
    assert!(!test(node, "b"));
  }

  #[test]
  fn repeat_test() {
    let node = &ast::repeat(ast::literal("a"));
    assert!(test(node, ""));
    assert!(test(node, "a"));
    assert!(test(node, "aa"));
    assert!(test(node, "aaa"));
    assert!(!test(node, "ba"));
    assert!(!test(node, "ab"));
  }
}
