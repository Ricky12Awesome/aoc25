#![allow(unused)]

use crate::prelude::*;
use itertools::Itertools;

pub fn run(input: &str, results: &mut OutputResults) -> anyhow::Result<()> {
    let input = input
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|s| (s.0.parse::<u128>().unwrap(), s.1.parse::<u128>().unwrap()))
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for (start, end) in input {
        for n in start..=end {
            let nums = n
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<u128>().unwrap())
                .collect_vec();

            if results.is_part1() && nums[0..nums.len() / 2].eq(&nums[nums.len() / 2..]) {
                p1 += n;
            }

            if results.is_part2() {
                for i in 1..nums.len() {
                    if nums.chunks(i).all_equal() {
                        p2 += n;
                        break;
                    }
                }
            }
        }
    }

    part1!(results, p1);
    part2!(results, p2);

    Ok(())
}
