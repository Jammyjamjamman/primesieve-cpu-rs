const BOOL_BLOCK: u64 = u64::BITS as u64;

fn sieve_of_eratosthenes_optimized(max: u32) -> Vec<u32> {
    let arr_sz = {
        let sz = (max as u64 + 1) / BOOL_BLOCK;
        if (max as u64 + 1) % BOOL_BLOCK == 0 {
            sz as usize
        }
        else {
            (sz + 1) as usize
        }
    };

    let mut sieve = vec![u64::MAX; arr_sz];
    sieve[0] ^= 3;

    for i in 2..=(max as f64).sqrt() as u64 {
        if (sieve[(i / BOOL_BLOCK) as usize] >> (i % BOOL_BLOCK)) & 1 == 1 {
            println!("{i}");
            if i >= 64 {
                // No more levareging parallelism of 64bit nums.
                let div = i / BOOL_BLOCK;
                let rem = i % BOOL_BLOCK;
                let start = i * i / BOOL_BLOCK;
                let mut cur_shift = i * i % BOOL_BLOCK;
                let mut idx_add = 0;
                let end = sieve.len() as u64;
                for j in (start..end).step_by(div as usize) {
                    if (j + idx_add) >= sieve.len() as u64 {
                        break;
                    }
                    sieve[(j + idx_add) as usize] &= !(1 << cur_shift);
                    cur_shift += rem;
                    if cur_shift >= BOOL_BLOCK {
                        cur_shift -= BOOL_BLOCK;
                        idx_add += 1;
                    }
                }
            }
            else {
                let batch_slice = batch_mark_slice(i);
                // start from the block the prime squared resides in.
                let from = ((i * i) / BOOL_BLOCK) / i;
                for j in from..=(sieve.len() as u64 / i) {
                    for k in 0..i as u64 {
                        if (j*i + k) >= sieve.len() as u64 {
                            break;
                        }
                        sieve[(j*i + k) as usize] &= batch_slice[k as usize];
                    }
                }
                // make sure the prime itself is still 1.
                sieve[(i / BOOL_BLOCK) as usize] |= 1 << i % BOOL_BLOCK;
            }
        }
    }

    (0..=max)
        .into_iter()
        .filter(|&v| (sieve[(v as u64 / BOOL_BLOCK) as usize] >> (v as u64 % BOOL_BLOCK)) & 1 == 1 )
        .collect()
}


fn batch_mark_slice(prime: u64) -> Vec<u64> {
    let mut batch_slice = vec![u64::MAX; prime as usize];
    for i in (prime..BOOL_BLOCK).step_by(prime as usize) {
        batch_slice[0] ^= 1 << i;
    }
    batch_slice[0] ^= 1;

    let diff = prime - BOOL_BLOCK % prime;

    for i in 1..prime as  usize {
        batch_slice[i] = batch_slice[i-1] << diff | batch_slice[i-1] >> prime - diff;
    }

    batch_slice
}

fn main() {
    let max = u32::MAX;
    let primes = sieve_of_eratosthenes_optimized(max);
    println!("Primes up to {}: {} Len: {}", max, primes.last().unwrap(), primes.len());
}
