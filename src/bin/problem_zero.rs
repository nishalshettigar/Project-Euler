fn main() {
    let n = 207_000;

    let sum: i64 = (1..=n)
        .map(|n| n * n)
        .filter(|n| n % 2 != 0)
        .sum();
    println!("{}", sum);
}
