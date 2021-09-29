use criterion::{Criterion, criterion_group, criterion_main, black_box};
use tiny_regex::ast;
use tiny_regex::ast::Node;

fn create_regex1() -> Node {
  ast::literal("test")
}

fn naive_benchmark(c: &mut Criterion) {
  use tiny_regex::matcher::naive;
  let node = create_regex1();
  c.bench_function(
    "naive: simple",
    |b| b.iter(|| {
      naive::test(&node, black_box("test"));
    })
  );
}

criterion_group!(benches, naive_benchmark);
criterion_main!(benches);
