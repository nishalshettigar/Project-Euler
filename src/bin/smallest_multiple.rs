use project_euler::primes::is_prime;

const LIMIT: u64 = 20;

fn main() {
    let mut smallest_multiple = 1;
    for i in 2..=LIMIT {
        if is_prime(i) {
            let mut j = 1;
            while (j * i) <= LIMIT {
                j *= i;
            }
            smallest_multiple *= j;
        }
    }
    println!("{smallest_multiple}");
}
