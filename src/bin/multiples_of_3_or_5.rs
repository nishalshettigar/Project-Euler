fn main() {
    let max = 1000;

    let sum = (1..max)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum::<i64>();

    println!("{}", sum);
}