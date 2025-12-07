#![allow(unused)]

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

mod prelude;

use crate::day::OutputResults;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(ValueEnum, Debug, Copy, Clone)]
#[repr(usize)]
enum ArgDay {
    #[clap(aliases = &["1"])]
    Day1 = 1,
    #[clap(aliases = &["2"])]
    Day2 = 2,
    #[clap(aliases = &["3"])]
    Day3 = 3,
    #[clap(aliases = &["4"])]
    Day4 = 4,
    #[clap(aliases = &["5"])]
    Day5 = 5,
    #[clap(aliases = &["6"])]
    Day6 = 6,
    #[clap(aliases = &["7"])]
    Day7 = 7,
    #[clap(aliases = &["8"])]
    Day8 = 8,
    #[clap(aliases = &["9"])]
    Day9 = 9,
    #[clap(aliases = &["10"])]
    Day10 = 10,
    #[clap(aliases = &["11"])]
    Day11 = 11,
    #[clap(aliases = &["12"])]
    Day12 = 12,
}

#[derive(ValueEnum, Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
enum ArgPart {
    #[clap(aliases = &["0", "all"])]
    #[default]
    Both = 0,
    #[clap(aliases = &["1", "one"])]
    Part1 = 1,
    #[clap(aliases = &["2", "two"])]
    Part2 = 2,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, short)]
    day: Option<ArgDay>,
    #[clap(long, short, default_value = "0")]
    part: ArgPart,
    #[clap(short, long)]
    bench: bool,
    #[clap(num_args = 0..=1)]
    input_path: Option<PathBuf>,
    #[clap(short)]
    input_string: Option<String>,
}

impl Args {
    fn input(&self) -> anyhow::Result<Option<String>> {
        if atty::isnt(atty::Stream::Stdin) {
            let mut input = String::new();

            std::io::stdin().read_to_string(&mut input)?;

            Ok(Some(input))
        } else {
            let Some(input) = self.input_path.as_ref() else {
                return Ok(self.input_string.clone());
            };

            let input = std::fs::read_to_string(input)?;

            Ok(Some(input))
        }
    }
}

fn main() {
    let args = Args::parse();

    let Some(input) = args.input().unwrap() else {
        Args::command()
            .error(ErrorKind::TooFewValues, "No input found")
            .exit();
    };

    let Some(day) = args.day.or_else(|| {
        args.input_path
            .map(|path| path.with_extension(""))
            .and_then(|path| ArgDay::from_str(path.file_name()?.to_str()?, true).ok())
    }) else {
        Args::command()
            .error(
                ErrorKind::InvalidValue,
                "Can't infer day from path or no day was given",
            )
            .exit();
    };

    if args.bench {
        if cfg!(debug_assertions) {
            Args::command()
                .error(
                    ErrorKind::Io,
                    "this argument can't run in debug mode, please use release mode.",
                )
                .exit();
        }

        let options = microbench::Options::default();

        let part1 = match day {
            ArgDay::Day1 => day01::bench_part1,
            ArgDay::Day2 => day02::bench_part1,
            ArgDay::Day3 => day03::bench_part1,
            ArgDay::Day4 => day04::bench_part1,
            ArgDay::Day5 => day05::bench_part1,
            ArgDay::Day6 => day06::bench_part1,
            ArgDay::Day7 => day07::bench_part1,
            ArgDay::Day8 => day08::bench_part1,
            ArgDay::Day9 => day09::bench_part1,
            ArgDay::Day10 => day10::bench_part1,
            ArgDay::Day11 => day11::bench_part1,
            ArgDay::Day12 => day12::bench_part1,
        };

        let part1 = || part1(&input);

        let part2 = match day {
            ArgDay::Day1 => day01::bench_part2,
            ArgDay::Day2 => day02::bench_part2,
            ArgDay::Day3 => day03::bench_part2,
            ArgDay::Day4 => day04::bench_part2,
            ArgDay::Day5 => day05::bench_part2,
            ArgDay::Day6 => day06::bench_part2,
            ArgDay::Day7 => day07::bench_part2,
            ArgDay::Day8 => day08::bench_part2,
            ArgDay::Day9 => day09::bench_part2,
            ArgDay::Day10 => day10::bench_part2,
            ArgDay::Day11 => day11::bench_part2,
            ArgDay::Day12 => day12::bench_part2,
        };

        let part2 = || part2(&input);

        if args.part == ArgPart::Part1 || args.part == ArgPart::Both {
            let name = format!("Day {} Part 1", day as usize);

            microbench::bench(&options, &name, part1);
        }

        if args.part == ArgPart::Part2 || args.part == ArgPart::Both {
            let name = format!("Day {} Part 2", day as usize);

            microbench::bench(&options, &name, part2);
        }

        return;
    }

    let mut results = OutputResults::from(args.part);

    let time = Instant::now();

    match day {
        ArgDay::Day1 => day01::run(&input, &mut results),
        ArgDay::Day2 => day02::run(&input, &mut results),
        ArgDay::Day3 => day03::run(&input, &mut results),
        ArgDay::Day4 => day04::run(&input, &mut results),
        ArgDay::Day5 => day05::run(&input, &mut results),
        ArgDay::Day6 => day06::run(&input, &mut results),
        ArgDay::Day7 => day07::run(&input, &mut results),
        ArgDay::Day8 => day08::run(&input, &mut results),
        ArgDay::Day9 => day09::run(&input, &mut results),
        ArgDay::Day10 => day10::run(&input, &mut results),
        ArgDay::Day11 => day11::run(&input, &mut results),
        ArgDay::Day12 => day12::run(&input, &mut results),
    };

    let time = time.elapsed();

    println!("{results}");
    println!("took {time:?}");
}
