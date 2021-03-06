use criterion::{Criterion, criterion_group, criterion_main, black_box};
use tiny_regex::ast::Node;

fn create_regex1() -> Node {
  use tiny_regex::ast::{literal, or, repeat, concat};
  let or = or(&[literal("a"), literal("b"), literal("c")]);
  let rep = repeat(or);
  concat(&[rep.clone(), rep.clone()])
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
  let program = vm::Program::new(&node);
  c.bench_function(
    "vm: simple",
    |b| b.iter(|| {
      vm::test(&program, black_box("aaaabbbb"));
    })
  );
}

criterion_group!(benches, naive_benchmark);
criterion_main!(benches);
