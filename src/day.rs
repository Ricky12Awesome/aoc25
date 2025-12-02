use crate::ArgPart;
use std::fmt::{Debug, Display};

pub struct Output(Box<dyn Display>);

impl Default for Output {
    fn default() -> Self {
        Self::new(Box::new(<&str>::default()))
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Output {
    pub fn new<T: Display + Debug + 'static>(value: T) -> Self {
        Output(Box::new(value))
    }
}

pub enum OutputResults {
    Both(Output, Output),
    Part1(Output),
    Part2(Output),
}

impl From<ArgPart> for OutputResults {
    fn from(value: ArgPart) -> Self {
        match value {
            ArgPart::Both => Self::Both(Output::default(), Output::default()),
            ArgPart::Part1 => Self::Part1(Output::default()),
            ArgPart::Part2 => Self::Part2(Output::default()),
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
    ($part:expr, $result:expr) => {
        match $part {
            $crate::day::OutputResults::Both(result, _) => {
                *result = $crate::day::Output::new($result);
            }
            $crate::day::OutputResults::Part1(result) => {
                *result = $crate::day::Output::new($result);
                return Ok(());
            }
            $crate::day::OutputResults::Part2(_) => {}
        }
    };
}

#[macro_export]
macro_rules! part2 {
    ($part:expr, $result:expr) => {
        match $part {
            $crate::day::OutputResults::Both(_, result) => {
                *result = $crate::day::Output::new($result);
            }
            $crate::day::OutputResults::Part1(_) => {}
            $crate::day::OutputResults::Part2(result) => {
                *result = $crate::day::Output::new($result);
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
