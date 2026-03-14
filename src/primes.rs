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

pub fn is_prime(n: u64) -> bool {
    for i in 2..=(n / 2) {
        if n % i == 0 {
            return false;
        }
    }
    n > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_number() {
        assert!(is_prime(2));
        assert!(is_prime(29));
    }

    #[test]
    fn non_primes() {
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(57));
    }
}
