fn main() {
    let max: u64 = 4_000_000;
    let mut n1 = 1;
    let mut n2 = 2;
    let mut sum = n2;
    while n2 <= max {
        let n = n1 + n2;
        n1 = n2;
        n2 = n;
        if n2 % 2 == 0 {
            sum += n2;
        }
    }
    println!("{}", sum);
}
