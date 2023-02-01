use crate::shared::math_helpers::is_prime_number;

/// Fibonacci sequence iterator.
/// - **Fibonacci sequence:** A series of numbers where the next term is found by adding up the two
///   previous terms.
pub struct FibonacciSequence {
    /// Term returned upon iteration.
    current_term: u64,

    /// Previously returned term.
    previous_term: u64,
}

impl Default for FibonacciSequence {
    fn default() -> Self {
        Self {
            current_term: 1,
            previous_term: 0,
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let temporary_term = self.current_term;
        self.current_term = self.current_term + self.previous_term;
        self.previous_term = temporary_term;

        Some(temporary_term)
    }
}

/// Prime numbers iterator.
pub struct PrimeNumbers {
    /// Prime number returned upon iteration.
    current_prime: u64,
}

impl Default for PrimeNumbers {
    fn default() -> Self {
        Self { current_prime: 2 }
    }
}

impl Iterator for PrimeNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_prime == 2 {
            self.current_prime = 1;

            return Some(2);
        }

        loop {
            self.current_prime += 2;

            if is_prime_number(self.current_prime) {
                return Some(self.current_prime);
            }
        }
    }
}
