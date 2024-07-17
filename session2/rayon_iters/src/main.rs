use rayon::prelude::*;
use std::time::Instant;

fn is_prime(n: u32) -> bool {
    (2..=n / 2).into_par_iter().all(|i| n % i != 0)
}

fn main() {
    let now = Instant::now();
    let numbers: Vec<u32> = (0..1_000).collect();
    let mut primes: Vec<&u32> = numbers.par_iter().filter(|n| is_prime(**n)).collect();
    primes.par_sort_unstable();
    let elapsed = now.elapsed().as_secs_f32();
    println!("Found {} primes in {} seconds", primes.len(), elapsed);
}
