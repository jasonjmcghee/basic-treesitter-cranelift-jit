use adder_treesitter_cranelift::language::Calculator;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration; // Adjust this based on your crate name

fn calculator_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculator");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Basic operations benchmarks
    let cases = vec![
        ("single_int", "42"),
        ("single_float", "42.5"),
        ("simple_add", "2 + 3"),
        ("simple_mul", "2 * 3"),
        ("mixed_types", "2 + 3.5"),
        ("complex_expr", "1 + 2 * 3 + 4.5"),
    ];

    for (name, input) in cases {
        group.bench_with_input(BenchmarkId::new("full_eval", name), input, |b, input| {
            let mut calc = Calculator::new().unwrap();
            b.iter(|| {
                calc.update_input(black_box(input), 0, 0, input.len())
                    .unwrap()
            });
        });
    }

    // Incremental update benchmarks
    group.bench_function("incremental_updates", |b| {
        let mut calc = Calculator::new().unwrap();
        b.iter(|| {
            calc.update_input("2", 0, 0, 1).unwrap();
            calc.update_input("2 ", 1, 1, 2).unwrap();
            calc.update_input("2 +", 2, 2, 3).unwrap();
            calc.update_input("2 + ", 3, 3, 4).unwrap();
            calc.update_input("2 + 3", 4, 4, 5).unwrap()
        });
    });

    // Cache effectiveness benchmarks
    group.bench_function("cache_hit", |b| {
        let mut calc = Calculator::new().unwrap();
        // Prime the cache
        calc.update_input("2 + 3", 0, 0, 5).unwrap();
        b.iter(|| calc.update_input(black_box("2 + 3"), 0, 0, 5).unwrap());
    });

    // Parser performance
    group.bench_function("parser_only", |b| {
        let mut calc = Calculator::new().unwrap();
        b.iter(|| calc.parser.parse(black_box("2 + 3 * 4"), None).unwrap());
    });

    // Compilation performance
    group.bench_function("compilation_only", |b| {
        let mut calc = Calculator::new().unwrap();
        let tree = calc.parser.parse("2 + 3 * 4", None).unwrap();
        let expr = calc.node_to_expr("2 + 3 * 4", tree.root_node()).unwrap();
        b.iter(|| {
            calc.compile_expr(black_box("2 + 3 * 4"), expr.clone())
                .unwrap()
        });
    });

    group.finish();
}

criterion_group!(benches, calculator_benchmarks);
criterion_main!(benches);
