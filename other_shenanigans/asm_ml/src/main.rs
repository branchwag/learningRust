// cargo run --release

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::hint::black_box;
use std::time::Instant;

fn dot_scalar(input: &[f32], weights: &[f32]) -> f32 {
    input.iter().zip(weights).map(|(x, w)| x * w).sum()
}

// Same idiomatic Rust as dot_scalar, but with AVX2+FMA enabled per-function —
// the per-fn equivalent of building with `-C target-cpu=native`. Gives LLVM
// every chance to autovectorize this loop.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
unsafe fn dot_scalar_autovec(input: &[f32], weights: &[f32]) -> f32 {
    input.iter().zip(weights).map(|(x, w)| x * w).sum()
}

#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
unsafe fn dot_intrinsics(input: &[f32], weights: &[f32]) -> f32 {
    unsafe {
        let mut s0 = _mm256_setzero_ps();
        let mut s1 = _mm256_setzero_ps();
        let mut s2 = _mm256_setzero_ps();
        let mut s3 = _mm256_setzero_ps();

        let x_ptr = input.as_ptr();
        let w_ptr = weights.as_ptr();
        let mut i = 0;

        while i + 32 <= input.len() {
            let x0 = _mm256_loadu_ps(x_ptr.add(i));
            let x1 = _mm256_loadu_ps(x_ptr.add(i + 8));
            let x2 = _mm256_loadu_ps(x_ptr.add(i + 16));
            let x3 = _mm256_loadu_ps(x_ptr.add(i + 24));
            let w0 = _mm256_loadu_ps(w_ptr.add(i));
            let w1 = _mm256_loadu_ps(w_ptr.add(i + 8));
            let w2 = _mm256_loadu_ps(w_ptr.add(i + 16));
            let w3 = _mm256_loadu_ps(w_ptr.add(i + 24));
            s0 = _mm256_fmadd_ps(x0, w0, s0);
            s1 = _mm256_fmadd_ps(x1, w1, s1);
            s2 = _mm256_fmadd_ps(x2, w2, s2);
            s3 = _mm256_fmadd_ps(x3, w3, s3);
            i += 32;
        }

        // 8-wide tail in groups of 8
        while i + 8 <= input.len() {
            let x = _mm256_loadu_ps(x_ptr.add(i));
            let w = _mm256_loadu_ps(w_ptr.add(i));
            s0 = _mm256_fmadd_ps(x, w, s0);
            i += 8;
        }

        let sum = _mm256_add_ps(_mm256_add_ps(s0, s1), _mm256_add_ps(s2, s3));
        let mut tmp = [0.0f32; 8];
        _mm256_storeu_ps(tmp.as_mut_ptr(), sum);
        let mut total: f32 = tmp.iter().sum();

        while i < input.len() {
            total += input[i] * weights[i];
            i += 1;
        }
        total
    }
}

// Four independent accumulators break the FMA latency chain (~4 cycle latency,
// 0.5 cycle throughput on Haswell+): with one accumulator the loop is latency-
// bound; with four it's throughput-bound. Compilers won't always do this for you.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "fma")]
unsafe fn dot_asm(input: &[f32], weights: &[f32]) -> f32 {
    let len = input.len();
    let chunks = len / 32;
    let processed = chunks * 32;
    let mut out_buf = [0.0f32; 8];

    if chunks > 0 {
        unsafe {
            core::arch::asm!(
                "vxorps ymm0, ymm0, ymm0",
                "vxorps ymm1, ymm1, ymm1",
                "vxorps ymm2, ymm2, ymm2",
                "vxorps ymm3, ymm3, ymm3",
                "xor rax, rax",
                "2:",
                "vmovups ymm4, [{x} + rax*4]",
                "vfmadd231ps ymm0, ymm4, [{w} + rax*4]",
                "vmovups ymm5, [{x} + rax*4 + 32]",
                "vfmadd231ps ymm1, ymm5, [{w} + rax*4 + 32]",
                "vmovups ymm6, [{x} + rax*4 + 64]",
                "vfmadd231ps ymm2, ymm6, [{w} + rax*4 + 64]",
                "vmovups ymm7, [{x} + rax*4 + 96]",
                "vfmadd231ps ymm3, ymm7, [{w} + rax*4 + 96]",
                "add rax, 32",
                "cmp rax, {n}",
                "jl 2b",
                "vaddps ymm0, ymm0, ymm1",
                "vaddps ymm2, ymm2, ymm3",
                "vaddps ymm0, ymm0, ymm2",
                "vmovups [{out}], ymm0",
                x = in(reg) input.as_ptr(),
                w = in(reg) weights.as_ptr(),
                n = in(reg) processed,
                out = in(reg) out_buf.as_mut_ptr(),
                out("rax") _,
                out("ymm0") _,
                out("ymm1") _,
                out("ymm2") _,
                out("ymm3") _,
                out("ymm4") _,
                out("ymm5") _,
                out("ymm6") _,
                out("ymm7") _,
                options(nostack),
            );
        }
    }

    let mut total: f32 = out_buf.iter().sum();
    for i in processed..len {
        total += input[i] * weights[i];
    }
    total
}

fn bench<F: FnMut() -> f32>(name: &str, iters: u32, mut f: F) -> (f64, f32) {
    for _ in 0..32 {
        black_box(f());
    }
    let start = Instant::now();
    let mut acc: f32 = 0.0;
    for _ in 0..iters {
        acc += black_box(f());
    }
    let elapsed = start.elapsed();
    let ns_per = elapsed.as_nanos() as f64 / iters as f64;
    let result = acc / iters as f32;
    println!("  {name:<12} {ns_per:>10.2} ns/iter   result = {result:.6}");
    (ns_per, result)
}

fn main() {
    if !is_x86_feature_detected!("avx2") || !is_x86_feature_detected!("fma") {
        eprintln!("CPU lacks AVX2 or FMA; this bench needs both.");
        return;
    }

    let n = 4096;
    let input: Vec<f32> = (0..n).map(|i| i as f32 / n as f32).collect();
    let weights: Vec<f32> = (0..n).map(|i| (i as f32).sin() * 0.01).collect();

    let iters = 200_000;
    println!("N = {n}, iters = {iters}\n");

    let (t_scalar, _) = bench("scalar", iters, || {
        dot_scalar(black_box(&input), black_box(&weights))
    });
    let (t_autovec, _) = bench("scalar+avx2", iters, || unsafe {
        dot_scalar_autovec(black_box(&input), black_box(&weights))
    });
    let (t_intr, _) = bench("intrinsics", iters, || unsafe {
        dot_intrinsics(black_box(&input), black_box(&weights))
    });
    let (t_asm, _) = bench("asm!", iters, || unsafe {
        dot_asm(black_box(&input), black_box(&weights))
    });

    println!("\nspeedup vs scalar:");
    println!("  scalar+avx2: {:.2}x", t_scalar / t_autovec);
    println!("  intrinsics:  {:.2}x", t_scalar / t_intr);
    println!("  asm!:        {:.2}x", t_scalar / t_asm);
    println!("speedup vs scalar+avx2 (the compiler's best autovec attempt):");
    println!("  intrinsics:  {:.2}x", t_autovec / t_intr);
    println!("  asm!:        {:.2}x", t_autovec / t_asm);
}
