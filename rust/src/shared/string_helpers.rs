/// Check if input string is a palindrome.
/// - **input_string:** Input string to check if it is a palindrome.
/// ## Notes
/// - **Palindrome:** A word, phrase, or sequence that reads the same backward as forward.
pub fn is_palindrome(input_string: String) -> bool {
    input_string == input_string.chars().rev().collect::<String>()
}
