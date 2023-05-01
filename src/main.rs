use std::fmt;
use std::str::FromStr;
use std::error::Error;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum IsbnError {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidChar(usize, char),
}

impl Error for IsbnError {}

impl fmt::Display for IsbnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ISBN Error: {:?}", self)
    }
}

impl FromStr for Isbn {
    type Err = IsbnError; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<u8> = Vec::new();
        for (i, ch) in s.char_indices() {
            if ch == '-' {
                continue;
            }
            let d: u8 = match ch.to_digit(10) {
                Some(x) => x as u8,
                None => return Err(IsbnError::InvalidChar(i, ch))
            };
            digits.push(d);
        }
        match digits.len() {
            0..=12 => Err(IsbnError::TooShort),
            13 => {
                if calculate_check_digit(&digits) == digits[12] {
                    Ok(Isbn { raw: s.to_string(), digits })
                } else {
                    Err(IsbnError::FailedChecksum)
                }
            },
            _ => Err(IsbnError::TooLong)
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let sum: u8 = digits.iter().zip(WEIGHTS).map(|(&a, b)| a*b).sum();
    (10 - (sum % 10)) % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
