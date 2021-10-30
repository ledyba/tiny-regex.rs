use criterion::{Criterion, criterion_group, criterion_main, black_box};
use tiny_regex::ast;
use tiny_regex::ast::Node;

fn create_regex1() -> Node {
  let or = ast::or(&[ast::literal("a"), ast::literal("b"), ast::literal("c")]);
  let rep = ast::repeat(or);
  ast::concat(&[rep.clone(), rep.clone()])
}

fn naive_benchmark(c: &mut Criterion) {
  use tiny_regex::engine::naive;
  use tiny_regex::engine::vm;
  let node = create_regex1();
  c.bench_function(
    "naive: simple",
    |b| b.iter(|| {
      naive::test(&node, black_box("aaaabbbb"));
    })
  );
  let codes = vm::compile(&node);
  println!("{}", codes);
  c.bench_function(
    "vm: simple",
    |b| b.iter(|| {
      vm::test(&codes, black_box("aaaabbbb"));
    })
  );
}

criterion_group!(benches, naive_benchmark);
criterion_main!(benches);
