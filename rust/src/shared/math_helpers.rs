use crate::shared::Exponent;

/// Check if input number is a prime number.
/// - **input_number:** Input number to check if it is a prime number.
/// ## Notes
/// - **Prime Number:** A number that has exactly two factors, 1 and itself.
pub fn is_prime_number(input_number: u64) -> bool {
    if input_number == 2 {
        return true;
    } else if input_number < 2 || input_number % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= input_number {
        if input_number % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

/// Get prime factors of input number using prime factorization.
/// - **input_number:** Input number to get prime factors for.
/// ## Notes
/// - **Prime Factor:** A prime number that divides evenly into a given number.
/// - **Prime Factorization:** The process of finding the prime factors of an integer, by dividing
///   it repeatedly by the smallest prime number that divides into it evenly.
pub fn prime_factors(mut input_number: u64) -> Vec<Exponent> {
    let mut prime_factors = vec![];

    if input_number == 0 {
        return prime_factors;
    }

    fn add_prime_factor(prime_factors: &mut Vec<Exponent>, base_value: u64) {
        match prime_factors
            .iter_mut()
            .find(|factor| factor.base_value == base_value)
        {
            Some(factor) => factor.power += 1,
            None => prime_factors.push(Exponent {
                base_value,
                power: 1,
            }),
        }
    }

    while input_number % 2 == 0 {
        add_prime_factor(&mut prime_factors, 2);
        input_number /= 2;
    }

    let mut i = 3;
    loop {
        while input_number % i == 0 {
            add_prime_factor(&mut prime_factors, i);
            input_number /= i;
        }

        if i as f32 > (input_number as f32).sqrt() {
            break;
        }

        i += 2;
    }

    if input_number > 2 {
        add_prime_factor(&mut prime_factors, input_number);
    }

    prime_factors
}

/// Get proper divisors of input number.
/// - **input_number:** Input number to get proper divisors for.
/// ## Notes
/// - **Proper Divisors:** Factors of a number excluding the number itself.
pub fn proper_divisors(input_number: u64) -> Vec<u64> {
    let mut proper_divisors = vec![];

    for i in 1..=(input_number as f64).sqrt().floor() as u64 {
        if input_number % i == 0 {
            proper_divisors.push(i);

            if input_number / i != i && i != 1 {
                proper_divisors.push(input_number / i);
            }
        }
    }

    proper_divisors.sort();

    proper_divisors
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::{is_prime_number, prime_factors, proper_divisors};
    use crate::shared::Exponent;
    use test::Bencher;

    #[test]
    fn is_prime_number_returns_true_for_prime_numbers() {
        let prime_numbers: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];

        for prime_number in prime_numbers {
            assert!(is_prime_number(prime_number));
        }
    }

    #[test]
    fn is_prime_number_returns_false_for_composite_numbers() {
        let composite_numbers: Vec<u64> = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18];

        for composite_number in composite_numbers {
            assert!(is_prime_number(composite_number) == false);
        }
    }

    #[test]
    fn prime_factors_returns_in_exponential_form() {
        assert_eq!(
            prime_factors(850),
            vec![
                Exponent {
                    base_value: 2,
                    power: 1,
                },
                Exponent {
                    base_value: 5,
                    power: 2,
                },
                Exponent {
                    base_value: 17,
                    power: 1,
                },
            ]
        );
    }

    #[test]
    fn proper_divisors_returns_proper_divisors_for_input_number() {
        assert_eq!(
            proper_divisors(220),
            vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]
        );
    }

    #[bench]
    fn bench_is_prime_number_using_one_millionth_prime_number(bencher: &mut Bencher) {
        bencher.iter(|| is_prime_number(15_485_863));
    }

    #[bench]
    fn bench_prime_factors_using_one_millionth_prime_number(bencher: &mut Bencher) {
        bencher.iter(|| prime_factors(15_485_863));
    }

    #[bench]
    fn bench_proper_divisors_using_one_million(bencher: &mut Bencher) {
        bencher.iter(|| proper_divisors(1_000_000));
    }
}
