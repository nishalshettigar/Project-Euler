use std::collections::HashMap;

pub fn prime_factorization(n: u64) -> HashMap<u64, u64> {
    let mut p = 2;
    let mut m = n;
    let mut prime_factors = HashMap::new();

    while m > 1 {
        while m % p == 0 {
            prime_factors.insert(p, prime_factors.get(&p).map_or(1, |&v| v + 1));
            m /= p;
        }
        p += 1;
    }
    prime_factors
}