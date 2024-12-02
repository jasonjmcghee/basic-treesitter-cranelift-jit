use adder_treesitter_cranelift::language::Calculator;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

#[derive(Clone)]
struct ExpressionConfig {
    max_depth: u32,
    max_terms: u32,
    allow_floats: bool,
    allow_parens: bool,
    allow_negatives: bool,
}

impl Default for ExpressionConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_terms: 5,
            allow_floats: true,
            allow_parens: true,
            allow_negatives: true,
        }
    }
}

fn generate_complex_expression(config: &ExpressionConfig) -> String {
    let mut rng = rand::thread_rng();
    generate_expression(config.max_depth, config, &mut rng)
}

fn generate_expression(depth: u32, config: &ExpressionConfig, rng: &mut impl rand::Rng) -> String {
    if depth == 0 || rng.gen_bool(0.3) {
        generate_number(config, rng)
    } else {
        if config.allow_parens && depth > 1 && rng.gen_bool(0.4) {
            format!("({})", generate_expression(depth - 1, config, rng))
        } else {
            let num_terms = rng.gen_range(2..=config.max_terms.min(5));
            let mut expr = generate_expression(depth - 1, config, &mut *rng);

            for _ in 1..num_terms {
                let operator = *["+", "-", "*", "/"].choose(rng).unwrap();
                expr.push_str(operator);
                expr.push_str(&generate_expression(depth - 1, config, rng));
            }
            expr
        }
    }
}

fn generate_number(config: &ExpressionConfig, rng: &mut impl rand::Rng) -> String {
    let is_float = config.allow_floats && rng.gen_bool(0.3);
    let is_negative = config.allow_negatives && rng.gen_bool(0.3);

    let num = if is_float {
        format!("{:.2}", rng.gen_range(0.0..1000.0))
    } else {
        rng.gen_range(0..1000).to_string()
    };

    if is_negative {
        format!("-{}", num)
    } else {
        num
    }
}

fn benchmark_calculator_update(c: &mut Criterion) {
    let mut calculator = Calculator::new().expect("Failed to create calculator");

    let config = ExpressionConfig {
        max_depth: 3,
        max_terms: 4,
        allow_floats: true,
        allow_parens: true,
        allow_negatives: true,
    };

    // Create a benchmark group for different expression complexities
    let mut group = c.benchmark_group("calculator_update");

    // Generate a set of test expressions beforehand
    let test_expressions: Vec<String> = (0..100)
        .map(|_| generate_complex_expression(&config))
        .collect();

    group.bench_function("update_input", |b| {
        let mut i = 0;
        b.iter(|| {
            let expr = &test_expressions[i % test_expressions.len()];
            i += 1;
            calculator.update_input(
                black_box(expr),
                black_box(0),
                black_box(0),
                black_box(expr.len()),
            )
        });
    });

    // Benchmark different expression complexities
    for depth in [1, 2, 3, 4] {
        let config = ExpressionConfig {
            max_depth: depth,
            ..config.clone()
        };

        let test_expressions: Vec<String> = (0..100)
            .map(|_| generate_complex_expression(&config))
            .collect();

        group.bench_function(format!("depth_{}", depth), |b| {
            let mut i = 0;
            b.iter(|| {
                let expr = &test_expressions[i % test_expressions.len()];
                i += 1;
                calculator.update_input(
                    black_box(expr),
                    black_box(0),
                    black_box(0),
                    black_box(expr.len()),
                )
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_calculator_update);
criterion_main!(benches);
