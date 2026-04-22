use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::io::Write;
use tempfile::{Builder, NamedTempFile};

use statline::io;
use statline::profiler::{self, ProfileOptions};

/// Generates a test dataset of a specific size with buffered I/O.
fn setup_benchmark_data(rows: usize) -> NamedTempFile {
    let mut file = Builder::new()
        .suffix(".csv")
        .tempfile()
        .expect("create temp csv");

    writeln!(file, "id,value,group").expect("write header");

    // Use a buffered writer approach to significantly speed up file generation
    let mut buffer = Vec::with_capacity(8192);
    for i in 0..rows {
        let v = (i as f64 * 0.137).sin() * 100.0;
        let g = i % 10;
        writeln!(&mut buffer, "{i},{v},{g}").expect("write row");

        if buffer.len() >= 8192 {
            file.write_all(&buffer).expect("write buffer");
            buffer.clear();
        }
    }

    // Flush remaining data
    if !buffer.is_empty() {
        file.write_all(&buffer).expect("write final buffer");
    }
    file.flush().expect("flush csv");

    file
}

fn bench_profile_csv_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("profile_csv");
    // Reduce sample size if larger datasets cause benchmark timeouts
    group.sample_size(10);

    // Test across an order of magnitude to establish performance scaling curves
    let sizes = [10_000, 50_000, 100_000];

    for size in sizes {
        // Data generation occurs strictly outside the measurement loop
        let temp_file = setup_benchmark_data(size);
        let path = temp_file.path().to_path_buf();

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_size| {
            b.iter(|| {
                let lf = io::scan_dataset(black_box(&path)).expect("scan dataset");

                let report = profiler::profile_dataset(
                    lf,
                    ProfileOptions {
                        include_correlation: true,
                        sample_rows: None,
                        include_insights: false,
                    },
                )
                .expect("profile dataset");

                black_box(report.rows)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_profile_csv_scaling);
criterion_main!(benches);
