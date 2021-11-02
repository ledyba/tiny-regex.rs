use crate::ast::Node;

#[derive(Debug)]
pub enum OpCode {
  Consume1(char),
  Consume(Vec<char>),
  Split(isize),
  Jump(isize),
  Fail,
}

impl std::fmt::Display for OpCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Consume1(s)  =>
        f.write_fmt(format_args!("Consume1 {}", s)),
      Self::Consume(s) =>
        f.write_fmt(format_args!("Consume  {:?}", s)),
      Self::Split(delta) =>
        f.write_fmt(format_args!("Fork     {:+}", delta)),
      Self::Jump(delta) =>
        f.write_fmt(format_args!("Jump     {:+}", delta)),
      Self::Fail =>
        f.write_fmt(format_args!("Fail")),
    }
  }
}


#[derive(Debug)]
pub struct Program {
  codes: Vec<OpCode>,
}

impl Program {
  pub fn new(node: &Node) -> Self {
    Self {
      codes: compile(node),
    }
  }
}

impl std::fmt::Display for Program {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "---\n").unwrap();
    for code in &self.codes {
      write!(f, "  {}\n", code).unwrap();
    }
    write!(f, "---").unwrap();
    Ok(())
  }
}

struct Thread {
  pc: usize,
  sp: usize,
}

struct Machine<'a> {
  chars: Vec<char>,
  threads: Vec<Thread>,
  program: &'a Program,
}

impl <'a> Machine<'a> {
  fn new(program: &'a Program, string: &'a str) -> Self {
    Self {
      chars: string.to_string().chars().collect(),
      threads: Vec::new(),
      program,
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
    let codes = &self.program.codes;
    let codes_len = self.program.codes.len();
    loop {
      if th.pc == codes_len {
        return th.sp == self.chars.len();
      }
      match &codes[th.pc] {
        OpCode::Consume1(chr) => {
          if self.chars.get(th.sp) == Some(chr) {
            th.pc += 1;
            th.sp += 1;
          } else {
            return false;
          }
        }
        OpCode::Consume(chars) => {
          for chr in chars {
            if self.chars.get(th.sp) == Some(chr) {
              th.sp += 1;
            } else {
              return false;
            }
          }
          th.pc += 1;
        }
        OpCode::Split(b) => {
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

pub fn test(program: &Program, string: &str) -> bool {
  Machine::new(program, string).start()
}

fn compile(node: &Node) -> Vec<OpCode> {
  match node {
    Node::Literal(literal) => {
      let chars = literal.chars().collect::<Vec<char>>();
      if chars.len() > 1 {
        vec![OpCode::Consume(chars)]
      } else {
        vec![OpCode::Consume1(*chars.get(0).unwrap())]
      }
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
      codes.push(OpCode::Split(0));

      let mut body = compile(&node);
      let body_len = body.len();
      codes.append(&mut body);

      codes.push(OpCode::Jump(-(body_len as isize) - 1));
      // Fix jump indecies
      codes[0] = OpCode::Split((body_len + 2) as isize);
      codes
    }
    Node::Or(noedes) => {
      let mut jmp_offsets = Vec::<usize>::new();
      let mut codes = Vec::<OpCode>::new();
      for node in noedes {
        let split_idx = codes.len();
        codes.push(OpCode::Split(0));
        let mut body = compile(node);
        let body_len = body.len();
        codes.append(&mut body);
        jmp_offsets.push(codes.len());
        codes.push(OpCode::Jump(0));
        codes[split_idx] = OpCode::Split((body_len as isize) + 2);
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
  use super::{test, Program};
  use crate::ast;

  #[test]
  fn literal_test() {
    let node = ast::literal("abc");
    let program = Program::new(&node);
    assert!(test(&program, "abc"));
    assert!(!test(&program, "ab"));
    assert!(!test(&program, "abcd"));
  }

  #[test]
  fn repeat_test() {
    let node = ast::repeat(ast::literal("a"));
    let program = Program::new(&node);
    assert!(test(&program, ""));
    assert!(test(&program, "a"));
    assert!(test(&program, "aa"));
    assert!(!test(&program, "ab"));
    assert!(!test(&program, "aab"));
    assert!(!test(&program, "baa"));
  }
  #[test]
  fn or_test() {
    let node = ast::or(&[ast::literal("a"), ast::literal("b"), ast::literal("c")]);
    let program = Program::new(&node);
    assert!(!test(&program, ""));
    assert!(test(&program, "a"));
    assert!(test(&program, "b"));
    assert!(test(&program, "c"));
    assert!(!test(&program, "d"));
  }
}