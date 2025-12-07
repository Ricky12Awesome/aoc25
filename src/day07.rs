use crate::prelude::*;
use std::collections::HashSet;

pub fn run(input: &str, results: &mut OutputResults) -> anyhow::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = grid[0].iter().position(|c| *c == 'S').unwrap();
    let mut beams = HashSet::from([start]);
    let mut sum = vec![0; grid[0].len()];
    sum[start] = 1;

    let mut p1 = 0;

    for row in grid.iter() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' && beams.contains(&x) {
                let curr = sum[x];
                sum[x + 1] += curr;
                sum[x - 1] += curr;
                sum[x] = 0;

                beams.remove(&x);
                beams.insert(x - 1);
                beams.insert(x + 1);
                p1 += 1;
            }
        }
    }

    part1!(results, "{p1}");
    part2!(results, "{}", sum.iter().sum::<usize>());
    Ok(())
}
