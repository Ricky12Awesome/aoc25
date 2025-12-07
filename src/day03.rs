use crate::prelude::*;

pub fn max(digits: &[u32], amount: usize) -> u128 {
    let mut last = 0;
    let mut result = 0;

    for i in 0..amount {
        let max = *digits[last..digits.len() - (amount - i) + 1]
            .iter()
            .max()
            .unwrap();

        let max_index = last + digits[last..].iter().position(|n| *n == max).unwrap();

        result += max as u128 * 10u128.pow(((amount - i) - 1) as _);

        last = max_index + 1;
    }

    result
}

pub fn run(input: &str, results: &mut OutputResults) {
    let (p1, p2) = input
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()))
        .map(|digits| digits.collect_vec())
        .fold((0u128, 0u128), |(mut p1, mut p2), digits| {
            if results.is_part1() {
                p1 += max(&digits, 2);
            }

            if results.is_part2() {
                p2 += max(&digits, 12);
            }

            (p1, p2)
        });

    // format!()

    part1!(results, "{p1}");
    part2!(results, "{p2}");
}

pub(crate) fn bench_part1(input: &str) {
    panic!("unimplemented");
}

pub(crate) fn bench_part2(input: &str) {
    panic!("unimplemented");
}