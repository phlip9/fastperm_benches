use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fastperm::{
    BitScatter, Shuffle, ShuffleArray, ShuffleArrayIncremental, SmallIndexPermutations, MAX_PERIOD,
};
use rand::SeedableRng;
use rand_xoshiro::Xoroshiro64Star;

const BENCH_SEED: u64 = 0xDEAD_BEEF_F000_BA55;

fn rng() -> Xoroshiro64Star {
    Xoroshiro64Star::seed_from_u64(BENCH_SEED)
}

fn index_permutation_bench(c: &mut Criterion) {
    let num_idxs: u8 = MAX_PERIOD;

    let mut shuffle = Shuffle::new(rng(), num_idxs);
    let mut shuffle_array = ShuffleArray::new(rng(), num_idxs);
    let mut shuffle_array_incr = ShuffleArrayIncremental::new(rng(), num_idxs);
    let mut bit_scatter = BitScatter::new(rng(), num_idxs);
    let mut rng2 = rng();
    let mut bit_scatter_rng_ref = BitScatter::new(&mut rng2, num_idxs);

    let mut g = c.benchmark_group("index_permutation_64_all");

    g.throughput(Throughput::Elements(num_idxs as u64));
    g.bench_function("shuffle", |b| {
        b.iter(|| {
            for _ in 0..num_idxs {
                shuffle.next_index();
            }
        })
    });
    g.bench_function("shuffle_array", |b| {
        b.iter(|| {
            for _ in 0..num_idxs {
                shuffle_array.next_index();
            }
        })
    });
    g.bench_function("shuffle_array_incr", |b| {
        b.iter(|| {
            for _ in 0..num_idxs {
                shuffle_array_incr.next_index();
            }
        })
    });
    g.bench_function("bit_scatter", |b| {
        b.iter(|| {
            for _ in 0..num_idxs {
                bit_scatter.next_index();
            }
        })
    });
    g.bench_function("bit_scatter_rng_ref", |b| {
        b.iter(|| {
            for _ in 0..num_idxs {
                bit_scatter_rng_ref.next_index();
            }
        })
    });

    g.finish();

    let mut g = c.benchmark_group("index_permutation_64_iter_period");

    g.throughput(Throughput::Elements(num_idxs as u64));
    g.bench_function("shuffle", |b| b.iter(|| for _ in shuffle.iter_period() {}));
    g.bench_function("shuffle_array", |b| {
        b.iter(|| for _ in shuffle_array.iter_period() {})
    });
    g.bench_function("shuffle_array_incr", |b| {
        b.iter(|| for _ in shuffle_array_incr.iter_period() {})
    });
    g.bench_function("bit_scatter", |b| {
        b.iter(|| for _ in bit_scatter.iter_period() {})
    });
    g.bench_function("bit_scatter_rng_ref", |b| {
        b.iter(|| for _ in bit_scatter_rng_ref.iter_period() {})
    });

    g.finish();

    let mut g = c.benchmark_group("index_permutation_64_one_idx");

    g.throughput(Throughput::Elements(1));
    g.bench_function("shuffle", |b| b.iter(|| shuffle.next_index()));
    g.bench_function("shuffle_array", |b| b.iter(|| shuffle_array.next_index()));
    g.bench_function("shuffle_array_incr", |b| {
        b.iter(|| shuffle_array_incr.next_index())
    });
    g.bench_function("bit_scatter", |b| b.iter(|| bit_scatter.next_index()));
    g.bench_function("bit_scatter_rng_ref", |b| {
        b.iter(|| bit_scatter_rng_ref.next_index())
    });

    g.finish();

    let mut g = c.benchmark_group("index_permutation_64_reset_and_one_idx");

    g.throughput(Throughput::Elements(1));
    g.bench_function("shuffle", |b| {
        b.iter(|| {
            shuffle.reset();
            shuffle.next_index();
        })
    });
    g.bench_function("shuffle_array", |b| {
        b.iter(|| {
            shuffle_array.reset();
            shuffle_array.next_index();
        })
    });
    g.bench_function("shuffle_array_incr", |b| {
        b.iter(|| {
            shuffle_array_incr.reset();
            shuffle_array_incr.next_index();
        })
    });
    g.bench_function("bit_scatter", |b| {
        b.iter(|| {
            bit_scatter.reset();
            bit_scatter.next_index();
        })
    });
    g.bench_function("bit_scatter_rng_ref", |b| {
        b.iter(|| {
            bit_scatter_rng_ref.reset();
            bit_scatter_rng_ref.next_index();
        })
    });

    g.finish();

    // let mut g = c.benchmark_group("index_permutation_64_take_8");
    //
    // g.throughput(Throughput::Elements(8));
    // g.bench_function("shuffle", |b| b.iter(|| for _ in shuffle.iter().take(8) {}));
    // g.bench_function("bit_scatter", |b| {
    //     b.iter(|| for _ in bit_scatter.iter().take(8) {})
    // });
    //
    // g.finish();

    // let num_idxs: u8 = 8;
    //
    // let rng1 = Xoroshiro64Star::seed_from_u64(BENCH_SEED);
    // let rng2 = Xoroshiro64Star::seed_from_u64(BENCH_SEED);
    // let mut shuffle = Shuffle::new(rng1, num_idxs);
    // let mut bit_scatter = BitScatter::new(rng2, num_idxs);
    //
    // let mut g = c.benchmark_group("index_permutation_8_all");
    //
    // g.throughput(Throughput::Elements(num_idxs as u64));
    // g.bench_function("shuffle", |b| b.iter(|| for _ in shuffle.iter_period() {}));
    // g.bench_function("bit_scatter", |b| {
    //     b.iter(|| for _ in bit_scatter.iter_period() {})
    // });
    //
    // g.finish();
    //
    // let mut g = c.benchmark_group("index_permutation_8_take_3");
    //
    // g.throughput(Throughput::Elements(3));
    // g.bench_function("shuffle", |b| b.iter(|| for _ in shuffle.iter().take(3) {}));
    // g.bench_function("bit_scatter", |b| {
    //     b.iter(|| for _ in bit_scatter.iter().take(3) {})
    // });
    //
    // g.finish();
}

criterion_group!(index_permutation_benches, index_permutation_bench);
criterion_main!(index_permutation_benches);
