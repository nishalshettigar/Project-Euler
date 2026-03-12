use project_euler::prime_factorization::prime_factorization;

const NUMBER: u64 = 600851475143;

fn main() {
    let prime_factors = prime_factorization(NUMBER);
    let largest_prime_factor = prime_factors
        .iter()
        .map(|(k, _)| *k)
        .max()
        .expect("No prime factors found");
    println!("{:?}", largest_prime_factor);
}
