use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn bench(c: &mut Criterion) {
    let mut x = 0x9E37_79B9u32;
    let data: Vec<u8> = (0..1 << 20)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            x as u8
        })
        .collect();

    let mut group = c.benchmark_group("adler32");
    for len in [16, 64, 1024, 16 << 10, 1 << 20] {
        let d = &data[..len];
        group.throughput(Throughput::Bytes(len as u64));
        group.bench_with_input(BenchmarkId::new("adler32furious", len), d, |b, d| {
            b.iter(|| adler32furious::adler32(d));
        });
        // Every implementation this CPU supports, not just the dispatched one.
        for imp in adler32furious::implementations() {
            let id = BenchmarkId::new(format!("adler32furious-{}", imp.name()), len);
            group.bench_with_input(id, d, |b, d| {
                b.iter(|| {
                    let mut sum = adler32furious::Adler32::new();
                    imp.update(&mut sum, d);
                    sum.checksum()
                });
            });
        }
        group.bench_with_input(BenchmarkId::new("libdeflate", len), d, |b, d| {
            b.iter(|| {
                let mut a = libdeflater::Adler32::new();
                a.update(d);
                a.sum()
            });
        });
        group.bench_with_input(BenchmarkId::new("simd-adler32", len), d, |b, d| {
            b.iter(|| simd_adler32::adler32(&d));
        });
        group.bench_with_input(BenchmarkId::new("adler2", len), d, |b, d| {
            b.iter(|| adler2::adler32_slice(d));
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
