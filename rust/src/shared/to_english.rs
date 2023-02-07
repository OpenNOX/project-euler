use num::Integer;
use std::fmt::Display;

/// Ones place values.
const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Tens place values.
const TENS: [&str; 18] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

/// Places values.
const PLACES: [&str; 22] = [
    "hundred",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
    "undecillion",
    "duodecillion",
    "tredecillion",
    "quattuordecillion",
    "quindecillion",
    "sexdecillion",
    "septendecillion",
    "octodecillion",
    "novemdecillion",
    "vigintillion",
];

/// Conversion trait used to extend number types to allow them to be spelled out in English.
pub trait ToEnglish {
    /// Get number spelled out in English.
    fn to_english(&self) -> String;
}

impl<T: Integer + Display> ToEnglish for T {
    fn to_english(&self) -> String {
        let digits_reversed: Vec<usize> = self
            .to_string()
            .chars()
            .rev()
            .map(|char| char.to_digit(10).expect("character to be base 10 digit") as usize)
            .collect();

        (0..digits_reversed.len())
            .enumerate()
            .fold(String::new(), |english, (i, _digit)| {
                if i % 3 != 0 {
                    return english;
                }

                let places_group_english = places_group_english(
                    i / 3,
                    &digits_reversed[i],
                    digits_reversed.get(i + 1),
                    digits_reversed.get(i + 2),
                );

                if places_group_english.is_empty() {
                    english
                } else {
                    format!("{} {}", places_group_english, english)
                }
            })
            .trim_end()
            .to_string()
    }
}

/// Get ones place digit spelled out in English.
/// - **ones_place_digit:** Ones place digit.
/// - **tens_place_digit:** Tens place digit.
fn ones_place_english<'a>(
    ones_place_digit: &'a usize,
    tens_place_digit: Option<&'a usize>,
) -> &'a str {
    if tens_place_digit.is_none() {
        ONES[*ones_place_digit]
    } else if *ones_place_digit != 0 && tens_place_digit.is_some_and(|digit| *digit != 1) {
        ONES[*ones_place_digit]
    } else {
        ""
    }
}

/// Get tens place digit spelled out in English.
/// - **ones_place_digit:** Ones place digit.
/// - **tens_place_digit:** Tens place digit.
fn tens_place_english(ones_place_digit: &usize, tens_place_digit: &usize) -> String {
    if *tens_place_digit == 1 {
        TENS[*ones_place_digit + *tens_place_digit - 1].to_string()
    } else if *tens_place_digit > 1 {
        let tens_place_english = TENS[9 + *tens_place_digit - 1];

        if *ones_place_digit != 0 {
            format!("{}-", tens_place_english)
        } else {
            tens_place_english.to_string()
        }
    } else {
        String::new()
    }
}

/// Get hundreds place digit spelled out in English.
/// - **ones_place_digit:** Ones place digit.
/// - **tens_place_digit:** Tens place digit.
/// - **hundreds_place_digit:** Hundreds place digit.
fn hundreds_place_english(
    ones_place_digit: &usize,
    tens_place_digit: &usize,
    hundreds_place_digit: &usize,
) -> String {
    if *hundreds_place_digit != 0 {
        if *tens_place_digit != 0 || *ones_place_digit != 0 {
            format!("{} {} and ", ONES[*hundreds_place_digit], PLACES[0])
        } else {
            format!("{} {}", ONES[*hundreds_place_digit], PLACES[0])
        }
    } else {
        String::new()
    }
}

/// Get concatenated places spelled out in English.
/// - **places_group_index** Places group index.
/// - **ones_place_digit:** Ones place digit.
/// - **tens_place_digit:** Tens place digit.
/// - **hundreds_place_digit:** Hundreds place digit.
fn places_group_english(
    places_group_index: usize,
    ones_place_digit: &usize,
    tens_place_digit: Option<&usize>,
    hundreds_place_digit: Option<&usize>,
) -> String {
    let mut places_group_english =
        ones_place_english(ones_place_digit, tens_place_digit).to_string();

    if places_group_index != 0
        && (*ones_place_digit != 0
            || tens_place_digit.is_some_and(|digit| *digit != 0)
            || hundreds_place_digit.is_some_and(|digit| *digit != 0))
    {
        places_group_english = format!("{} {}", places_group_english, PLACES[places_group_index]);
    }

    if tens_place_digit.is_some() {
        places_group_english = format!(
            "{}{}",
            tens_place_english(ones_place_digit, tens_place_digit.expect("to be some")),
            places_group_english
        );
    }

    if hundreds_place_digit.is_some() {
        places_group_english = format!(
            "{}{}",
            hundreds_place_english(
                ones_place_digit,
                tens_place_digit.expect("to be some since hundreds place digit is some"),
                hundreds_place_digit.expect("to be some")
            ),
            places_group_english
        );
    }

    places_group_english
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::ToEnglish;
    use test::Bencher;

    #[test]
    fn to_english_returns_number_spelled_out_in_english() {
        assert_eq!(420.to_english(), "four hundred and twenty");
    }

    #[test]
    fn to_english_returns_number_with_edge_cases_spelled_out_in_english() {
        assert_eq!(
            400_020_169_000_013_u64.to_english(),
            "four hundred trillion twenty billion one hundred and sixty-nine million thirteen"
        );
    }

    #[test]
    fn to_english_handles_zero() {
        assert_eq!(0.to_english(), "zero");
    }

    #[bench]
    fn bench_to_english_using_maximum_u128(bencher: &mut Bencher) {
        bencher.iter(|| u128::MAX.to_english());
    }
}
