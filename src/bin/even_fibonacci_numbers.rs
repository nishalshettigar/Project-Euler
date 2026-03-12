use std::iter::successors;

fn main() {
    let sum = successors(Some((1, 2)), |(a, b)| Some((*b, a + b)))
        .map(|(_, n)| n)
        .take_while(|n| *n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum::<u64>();
    println!("{sum}");
}
