use crate::prelude::*;

use std::ops::Deref;

pub fn run(input: &str, results: &mut OutputResults) -> anyhow::Result<()> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        .collect_vec();

    let p1 = ids
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|i| ranges.iter().any(|range| range.contains(i)))
        .count();

    part1!(results, "{p1}");

    ranges.sort_by_key(|range| *range.start());

    let mut merged = vec![ranges[0].clone()];

    for current in &ranges[1..] {
        let previous = merged.last_mut().unwrap();

        if previous.deref().contains(current.start()) && previous.deref().contains(current.end()) {
            continue;
        }

        if previous.deref().contains(current.start()) {
            *previous = *previous.start()..=*current.end();
        } else {
            merged.push(current.clone());
        }
    }

    ranges = merged;

    let p2 = ranges
        .iter()
        .map(|range| (range.end() - range.start()) + 1)
        .sum::<u64>();

    part2!(results, "{p2}");

    Ok(())
}
