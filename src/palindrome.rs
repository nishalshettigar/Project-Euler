pub fn is_palindrome(n: u64) -> bool {
    n == reverse(n)
}

fn reverse(n: u64) -> u64 {
    let mut m = n;
    let mut rev = 0;
    while m > 0 {
        let digit = m % 10;
        rev = rev * 10 + digit;
        m /= 10;
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit_number_is_a_palindrome() {
        let n = 6;

        assert_eq!(is_palindrome(n), true);
    }

    #[test]
    fn double_digit_number_same_digits_is_a_palindrome() {
        let n = 55;

        assert_eq!(is_palindrome(n), true);
    }

    #[test]
    fn double_digit_number_different_digits_not_a_palindrome() {
        let n = 35;

        assert_eq!(is_palindrome(n), false);
    }

    #[test]
    fn triple_digit_number_same_first_and_last_digit_is_a_palindrome() {
        let n = 353;

        assert_eq!(is_palindrome(n), true);
    }

    #[test]
    fn triple_digit_number_different_first_and_last_digit_is_not_a_palindrome() {
        let n = 553;

        assert_eq!(is_palindrome(n), false);
    }

    #[test]
    fn is_palindrome_not_a_palindrome() {
        let n = 2355;

        assert_eq!(is_palindrome(n), false);
    }

    #[test]
    fn is_palindrome_large_palindrome_number() {
        let n = 9870789;

        assert_eq!(is_palindrome(n), true);
    }
}
