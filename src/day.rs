use std::fmt::Display;
use crate::ArgPart;

pub enum OutputResults {
    Both(String, String),
    Part1(String),
    Part2(String),
}

#[allow(unused)]
impl OutputResults {
    pub fn is_part1(&self) -> bool {
        matches!(self, OutputResults::Both(_, _) | OutputResults::Part1(_))
    }

    pub fn is_part2(&self) -> bool {
        matches!(self, OutputResults::Both(_, _) | OutputResults::Part2(_))
    }
}

unsafe impl Send for OutputResults {}
unsafe impl Sync for OutputResults {}

impl From<ArgPart> for OutputResults {
    fn from(value: ArgPart) -> Self {
        match value {
            ArgPart::Both => Self::Both(String::default(), String::default()),
            ArgPart::Part1 => Self::Part1(String::default()),
            ArgPart::Part2 => Self::Part2(String::default()),
        }
    }
}

impl Display for OutputResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputResults::Both(a, b) => {
                writeln!(f, "Part 1: {a}")?;
                writeln!(f, "Part 2: {b}")
            }
            OutputResults::Part1(a) => writeln!(f, "Part 1: {a}"),
            OutputResults::Part2(b) => writeln!(f, "Part 2: {b}"),
        }
    }
}

#[macro_export]
macro_rules! part1 {
    ($part:expr, $($arg:tt)*) => {
        match $part {
            $crate::day::OutputResults::Both(result, _) => {
                *result = format!($($arg)*);
            }
            $crate::day::OutputResults::Part1(result) => {
                *result = format!($($arg)*);
                return Ok(());
            }
            $crate::day::OutputResults::Part2(_) => {}
        }
    };
}

#[macro_export]
macro_rules! part2 {
    ($part:expr, $($arg:tt)*) => {
        match $part {
            $crate::day::OutputResults::Both(_, result) => {
                *result = format!($($arg)*);
            }
            $crate::day::OutputResults::Part1(_) => {}
            $crate::day::OutputResults::Part2(result) => {
                *result = format!($($arg)*);
                return Ok(());
            }
        }
    };
}

// #[macro_export]
// macro_rules! day {
//     (|$input:ident, $results:ident| $body:block) => {
//         #[allow(unused)]
//         pub fn run($input: &str, $results: &mut $crate::day::OutputResults) -> ::anyhow::Result<()> $body
//     };
// }
