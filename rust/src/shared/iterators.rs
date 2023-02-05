use crate::shared::math_helpers::is_prime_number;

/// Collatz sequence iterator.
/// ## Notes
/// - **Collatz sequence:** A series of numbers where each term is found using a rule below until 1
///   is reached.
///   - **n is even:** n / 2
///   - **n is odd:** 3n + 1
pub struct CollatzSequence {
    /// Term returned upon iteration.
    current_term: Option<u64>,
}

impl CollatzSequence {
    pub fn new(initial_term: u64) -> Self {
        Self {
            current_term: Some(initial_term),
        }
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current_term = match self.current_term {
            Some(current_term) => current_term,
            None => return None,
        };

        self.current_term = if current_term == 1 {
            None
        } else if current_term % 2 == 0 {
            Some(current_term / 2)
        } else {
            Some((current_term * 3) + 1)
        };

        Some(current_term)
    }
}

/// Fibonacci sequence iterator.
/// ## Notes
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

#[cfg(test)]
mod tests {
    extern crate test;

    use super::{CollatzSequence, FibonacciSequence, PrimeNumbers};
    use test::Bencher;

    #[test]
    fn fibonacci_sequence_iterates_through_terms() {
        let expected_fibonacci_sequence: Vec<u64> = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        let actual_fibonacci_sequence: Vec<u64> = FibonacciSequence::default()
            .take(expected_fibonacci_sequence.len())
            .collect();

        assert_eq!(actual_fibonacci_sequence, expected_fibonacci_sequence);
    }

    #[test]
    fn prime_numbers_iterates_through_primes() {
        let expected_prime_numbers: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        let actual_prime_numbers: Vec<u64> = PrimeNumbers::default()
            .take(expected_prime_numbers.len())
            .collect();

        assert_eq!(actual_prime_numbers, expected_prime_numbers);
    }

    #[test]
    fn collatz_sequence_iterates_through_terms() {
        let expected_collatz_sequence_terms: Vec<u64> = vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1];
        let actual_collatz_sequence_terms: Vec<u64> = CollatzSequence::new(13).collect();

        assert_eq!(
            expected_collatz_sequence_terms,
            actual_collatz_sequence_terms
        );
    }

    #[bench]
    fn bench_fibonacci_sequence_to_fiftieth_term(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut fibonacci_sequence = FibonacciSequence::default();
            fibonacci_sequence
                .advance_by(50)
                .expect("fibonacci sequence iterator to be infinite");
        });
    }

    #[bench]
    fn bench_prime_numbers_to_one_thousandth_prime(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut prime_numbers = PrimeNumbers::default();
            prime_numbers
                .advance_by(1_000)
                .expect("prime numbers iterator to be infinite");
        });
    }

    #[bench]
    fn bench_collatz_sequence_using_one_thousand_as_a_start_number(bencher: &mut Bencher) {
        bencher.iter(|| CollatzSequence::new(1_000).count());
    }
}
