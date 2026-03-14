use project_euler::palindrome::is_palindrome;

fn main() {
    let mut max = 0;
    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let product = i * j;
            if (product > max) && is_palindrome(product) {
                max = product;
            }
        }
    }
    println!("{max}");
}
