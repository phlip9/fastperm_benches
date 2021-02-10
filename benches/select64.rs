use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fastperm::{
    fill_mask,
    select64::{pdep32, pdep32_fallback, select64, select64_fallback, select64_via_pdep32},
    BitScatter, SmallIndexPermutations, MAX_PERIOD,
};
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};

const NUM_INPUTS: usize = 1024;
const BENCH_SEED: u64 = 0xDEAD_BEEF_F000_BA55;

fn sample_idx_and_mask64<R: Rng>(rng: &mut R) -> (u8, u64) {
    let idx = rng.gen_range(0..MAX_PERIOD);
    let mask_bits = rng.gen_range(idx + 1..=MAX_PERIOD);
    let mut bit_scatter = BitScatter::new(rng, MAX_PERIOD);
    let bits_perm = bit_scatter.iter().take(mask_bits as usize);
    let mask = fill_mask(bits_perm).0;
    (idx, mask)
}

fn pdep32_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(BENCH_SEED);

    let inputs = (0..NUM_INPUTS)
        .into_iter()
        .map(|_| (rng.next_u32(), rng.next_u32()))
        .collect::<Vec<_>>();

    let mut g = c.benchmark_group("pdep32");
    g.throughput(Throughput::Elements(inputs.len() as u64));

    g.bench_with_input("pdep32", &inputs[..], |b, inputs| {
        b.iter(|| {
            for &(src, mask) in inputs {
                pdep32(src, mask);
            }
        })
    });
    g.bench_with_input("pdep32_fallback", &inputs[..], |b, inputs| {
        b.iter(|| {
            for &(src, mask) in inputs {
                pdep32_fallback(src, mask);
            }
        })
    });

    g.finish();
}

fn pdep32_bench_one_input(c: &mut Criterion) {
    let src = 0x5555_3333_u32;
    let mask = 0xF0F0_550F_u32;

    let mut g = c.benchmark_group("pdep32_one_input");

    g.bench_function("pdep32", |b| {
        b.iter(|| pdep32(black_box(src), black_box(mask)))
    });
    g.bench_function("pdep32_fallback", |b| {
        b.iter(|| {
            pdep32_fallback(black_box(src), black_box(mask));
        })
    });

    g.finish();
}

fn select64_bench(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(BENCH_SEED);

    let inputs = (0..NUM_INPUTS)
        .into_iter()
        .map(|_| sample_idx_and_mask64(&mut rng))
        .collect::<Vec<_>>();

    let mut g = c.benchmark_group("select64");
    g.throughput(Throughput::Elements(inputs.len() as u64));

    g.bench_with_input("select64", &inputs[..], |b, inputs| {
        b.iter(|| {
            for &(idx, mask) in inputs {
                select64(idx, mask);
            }
        })
    });
    g.bench_with_input("select64_via_pdep32", &inputs[..], |b, inputs| {
        b.iter(|| {
            for &(idx, mask) in inputs {
                select64_via_pdep32(idx, mask);
            }
        })
    });
    g.bench_with_input("select64_fallback", &inputs[..], |b, inputs| {
        b.iter(|| {
            for &(src, mask) in inputs {
                select64_fallback(src, mask);
            }
        })
    });

    g.finish();
}

criterion_group!(
    select64_benches,
    pdep32_bench,
    pdep32_bench_one_input,
    select64_bench
);
criterion_main!(select64_benches);
