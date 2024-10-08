use criterion::{criterion_group, criterion_main, Criterion};
use gruphst:: exporter_importer::csv::import_from_csv_gruphst_format;

fn import_csv_file(csv_file_path: &str) {
    let _graphs = import_from_csv_gruphst_format(csv_file_path).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let csv_file_path = "./benches/data/exported.csv";
    c.bench_function("import_CSV_file", |b| b.iter(|| import_csv_file(csv_file_path)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
