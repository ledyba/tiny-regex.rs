use crate::ast::Node;
pub enum OpCode {
  Consume(char),
  Fork(isize),
  Jump(isize),
}

struct Thread {
  pc: usize,
  sp: usize,
}

struct Machine<'a> {
  string: Vec<char>,
  str_len: usize,
  threads: Vec<Thread>,
  codes: &'a Vec<OpCode>,
  codes_len: usize,
}

impl <'a> Machine<'a> {
  fn new(codes: &'a Vec<OpCode>, string: &'a str) -> Self {
    let string: Vec<char> = string.chars().collect();
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
      if th.sp >= self.str_len {
        return false;
      }
      match &self.codes[th.pc] {
        OpCode::Consume(ch) => {
          if *ch == self.string[th.sp] {
            th.sp += 1;
            th.pc += 1;
            continue;
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
    Node::Repeat(node) => {
      let mut codes = Vec::new();
      codes.push(OpCode::Fork(0));
      let mut body = compile(&node);
      let body_len = body.len();
      codes.append(&mut body);
      codes.push(OpCode::Jump(-(body_len as isize) - 1));
      // Fix jump indecies
      codes[0] = OpCode::Fork((body_len + 1) as isize);
      codes
    }
    _ => {
      unimplemented!();
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
}