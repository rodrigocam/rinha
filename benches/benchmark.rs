use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/eval.rs"]
mod eval;

#[path = "../src/ast.rs"]
mod ast;

use crate::eval::Interpreter;

fn run_json(file_path: &str) {
    let ast = fs::read_to_string(file_path).expect("Failed to parse json");
    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("calling fib.json", |b| {
        b.iter(|| run_json(black_box("examples/fib.json")))
    });
}
// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }
//
// fn run_fib(n: u64) {
//     println!("{}", fibonacci(n))
// }
//
// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| run_fib(black_box(20))));
// }

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
