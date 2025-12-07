#![allow(unused)]

use crate::prelude::*;

pub fn run(input: &str, results: &mut OutputResults) {
    let input = input
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|s| (s.0.parse::<u128>().unwrap(), s.1.parse::<u128>().unwrap()))
        .collect::<Vec<_>>();

    let (p1, p2) = input
        .into_par_iter()
        .flat_map(|(start, end)| {
            (start..=end).into_par_iter().map(|n| {
                (
                    n,
                    n.to_string()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect_vec(),
                )
            })
        })
        .fold(
            || (0, 0),
            |(mut p1, mut p2), (n, digits)| {
                if digits[0..digits.len() / 2].eq(&digits[digits.len() / 2..]) {
                    p1 += n;
                }

                for i in 1..digits.len() {
                    if digits.chunks(i).all_equal() {
                        p2 += n;
                        break;
                    }
                }

                (p1, p2)
            },
        )
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| ((a1 + b1), (a2 + b2)));

    part1!(results, "{p1}");
    part2!(results, "{p2}");
}

pub(crate) fn bench_part1(input: &str) {
    panic!("unimplemented");
}

pub(crate) fn bench_part2(input: &str) {
    panic!("unimplemented");
}