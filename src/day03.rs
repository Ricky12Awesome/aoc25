#![allow(unused)]

use crate::prelude::*;
use itertools::Itertools;

// wrong 17333
pub fn run(input: &str, results: &mut OutputResults) -> anyhow::Result<()> {
    let p1 = input
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()))
        .map(|digits| digits.collect_vec())
        .map(|digits| {
            let largest1 = digits[..digits.len() - 1].iter().max().unwrap();
            let largest1_index = digits.iter().position(|n| *n == *largest1).unwrap();
            let largest2_index = largest1_index + 1 + digits[largest1_index + 1..].iter().position_max().unwrap();

            (digits[largest1_index] * 10) + digits[largest2_index]
        })
        .sum::<u32>();

    let p2 = input
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()))
        .map(|digits| digits.collect_vec())
        .map(|mut digits| {
            let amount = 12;
            let mut last = 0;
            let mut result = 0;

            for i in 0..amount {
                let max = *digits[last..digits.len() - (amount - i) + 1].iter().max().unwrap();
                let max_index = last + digits[last..].iter().position(|n| *n == max).unwrap();

                result += max as u128 * 10u128.pow(((amount  - i) - 1) as _);

                last = max_index + 1;
            }

            result
        })
        .sum::<u128>();

    part1!(results, p1);
    part2!(results, p2);

    Ok(())
}
