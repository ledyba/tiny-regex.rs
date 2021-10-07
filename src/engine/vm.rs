use crate::ast::Node;
pub enum OpCode {
  Consume(String),
  Fork(isize),
  Jump(isize),
  Fail,
}

struct Thread {
  pc: usize,
  sp: usize,
}

struct Machine<'a> {
  string: String,
  str_len: usize,
  threads: Vec<Thread>,
  codes: &'a Vec<OpCode>,
  codes_len: usize,
}

impl <'a> Machine<'a> {
  fn new(codes: &'a Vec<OpCode>, string: &'a str) -> Self {
    let string = string.to_string();
    let str_len = string.len();
    Self {
      string,
      str_len,
      threads: Vec::new(),
      codes,
      codes_len: codes.len(),
    }
  }
  fn start(&mut self) -> bool {
    self.threads.push(Thread {
      pc: 0,
      sp: 0,
    });
    while !self.threads.is_empty() {
      if self.schedule_thread() {
        return true;
      }
    }
    false
  }
  fn schedule_thread(&mut self) -> bool {
    let mut th = &mut self.threads.pop().expect("No threads");
    loop {
      if th.pc == self.codes_len {
        return th.sp == self.str_len;
      }
      match &self.codes[th.pc] {
        OpCode::Consume(str) => {
          let str_len = str.len();
          if th.sp + str_len <= self.str_len {
            if self.string[th.sp..].starts_with(str) {
              th.sp += str_len;
              th.pc += 1;
              continue;
            }
          }
          return false;
        }
        OpCode::Fork(b) => {
          self.threads.push(Thread{
            pc: ((th.pc as isize) + *b) as usize,
            sp: th.sp,
          });
          th.pc += 1;
        }
        OpCode::Jump(n) => {
          th.pc = ((th.pc as isize) + *n) as usize;
        }
        OpCode::Fail => {
          return false;
        }
      }
    }
  
  }
}

pub fn test(codes: &Vec<OpCode>, string: &str) -> bool {
  Machine::new(codes, string).start()
}

pub fn compile(node: &Node) -> Vec<OpCode> {
  match node {
    Node::Literal(literal) => {
      vec![OpCode::Consume(literal.clone())]
    }
    Node::Concat(nodes) => {
      nodes
        .iter()
        .map(|node| compile(node))
        .flatten()
        .collect()
    }
    Node::Repeat(node) => {
      let mut codes = Vec::new();
      codes.push(OpCode::Fork(0));

      let mut body = compile(&node);
      let body_len = body.len();
      codes.append(&mut body);

      codes.push(OpCode::Jump(-(body_len as isize) - 1));
      // Fix jump indecies
      codes[0] = OpCode::Fork((body_len + 2) as isize);
      codes
    }
    Node::Or(noedes) => {
      let mut jmp_offsets = Vec::<usize>::new();
      let mut codes = Vec::<OpCode>::new();
      for node in noedes {
        let current = codes.len();
        codes.push(OpCode::Fork(0));
        let mut body = compile(node);
        let body_len = body.len();
        codes.append(&mut body);
        jmp_offsets.push(codes.len());
        codes.push(OpCode::Jump(0));
        codes[current] = OpCode::Fork((body_len as isize) + 2);
      }
      codes.push(OpCode::Fail);
      let codes_len = codes.len();
      for offset in &jmp_offsets {
        codes[*offset] = OpCode::Jump((codes_len - offset) as isize);
      }
      codes
    }
  }
}

#[cfg(test)]
mod test {
  use super::{test, compile};
  use crate::{ast};

  #[test]
  fn literal_test() {
    let node = ast::literal("abc");
    let codes = compile(&node);
    assert!(test(&codes, "abc"));
    assert!(!test(&codes, "ab"));
    assert!(!test(&codes, "abcd"));
  }

  #[test]
  fn repeat_test() {
    let node = ast::repeat(ast::literal("a"));
    let codes = compile(&node);
    assert!(test(&codes, ""));
    assert!(test(&codes, "a"));
    assert!(test(&codes, "aa"));
    assert!(!test(&codes, "ab"));
    assert!(!test(&codes, "aab"));
    assert!(!test(&codes, "baa"));
  }
  #[test]
  fn or_test() {
    let node = ast::or(&[ast::literal("a"), ast::literal("b"), ast::literal("c")]);
    let codes = compile(&node);
    assert!(!test(&codes, ""));
    assert!(test(&codes, "a"));
    assert!(test(&codes, "b"));
    assert!(test(&codes, "c"));
    assert!(!test(&codes, "d"));
  }
}