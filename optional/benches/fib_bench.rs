//! Benchmarking example from: https://bheisler.github.io/criterion.rs/book/getting_started.html

use criterion::{black_box, criterion_group, criterion_main, Criterion};

{% if is_library %}
use {{project-name}}::calculate_fib;{% else %}
pub fn calculate_fib(n: u64) -> u64 {
    {% if use_flamegraph %}
    #[cfg(feature = "flame_on")]
    let _fg = flame::start_guard(format!("{}::calculate_fib({})", module_path!(), n));
    {% endif %}
    match n {
        0 => 0,
        1 => 1,
        n => calculate_fib(n - 1) + calculate_fib(n - 2),
    }
}{% endif %}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| calculate_fib(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
