/// Check if input string is a palindrome.
/// - **input_string:** Input string to check if it is a palindrome.
/// ## Notes
/// - **Palindrome:** A word, phrase, or sequence that reads the same backward as forward.
pub fn is_palindrome(input_string: &str) -> bool {
    input_string == input_string.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::is_palindrome;
    use test::Bencher;

    #[test]
    fn is_palindrome_returns_true_when_palindrome() {
        assert!(is_palindrome("9009"));
    }

    #[test]
    fn is_palindrome_returns_false_when_not_palindrome() {
        assert!(is_palindrome("8989") == false);
    }

    #[bench]
    fn bench_is_palindrome_using_one_hundred_character_string(bencher: &mut Bencher) {
        let input_string = "8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008";

        bencher.iter(|| is_palindrome(input_string));
    }
}
