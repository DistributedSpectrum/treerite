use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::array;
use treerite::GTIConfig;

#[path = "../examples/support/mod.rs"]
mod support;

fn simple_bench(c: &mut Criterion) {
    let model = support::build_model();
    let config = GTIConfig::parse(r#"{"predict_type":"default","nthread":1}"#).unwrap();
    let input = array![7.7f64, 2.8];
    c.bench_function("single_row", |b| {
        b.iter(|| model.predict(&config, black_box(&input), Some(1)).unwrap())
    });
}

criterion_group!(benches, simple_bench);
criterion_main!(benches);
