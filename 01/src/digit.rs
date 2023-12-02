use std::fmt::Display;
use std::slice::Iter;
use crate::digit::Digit::*;

#[derive(Debug)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Digit {
    pub fn iterator() -> Iter<'static, Digit> {
        static DIGITS: [Digit; 10] = [
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine
        ];
        DIGITS.iter()
    }

    pub fn as_protected_num(&self) -> String {
        match self {
            Zero => "zero0zero",
            One => "one1one",
            Two => "two2two",
            Three => "three3three",
            Four => "four4four",
            Five => "five5five",
            Six => "six6six",
            Seven => "seven7seven",
            Eight => "eight8eight",
            Nine => "nine9nine",
        }.to_string()
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Zero => "zero",
            One => "one",
            Two => "two",
            Three => "three",
            Four => "four",
            Five => "five",
            Six => "six",
            Seven => "seven",
            Eight => "eight",
            Nine => "nine",
        };
        write!(f, "{}", str)
    }
}
