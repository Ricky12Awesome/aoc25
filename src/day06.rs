#![allow(unused)]
use crate::prelude::*;

pub fn run(str: &str, results: &mut OutputResults) -> anyhow::Result<()> {
    let mut input = str.trim().rsplit("\n");
    let ops = input.next().unwrap();
    let ops = ops.split_whitespace().collect_vec();

    let mut input = input //
        .map(|line| line.split_whitespace().map(|s| s.parse::<u64>().unwrap()))
        .fold(vec![Vec::new(); ops.len()], |mut acc, line| {
            for (i, n) in line.enumerate() {
                acc[i].push(n);
                acc[i].sort();
                acc[i].reverse();
            }

            acc
        });

    let p1 = input
        .iter()
        .enumerate()
        .fold(0, |acc, (i, col)| match ops[i] {
            "+" => acc + col.iter().sum::<u64>(),
            "*" => acc + col.iter().product::<u64>(),
            _ => unreachable!(),
        });

    part1!(results, "{p1}");

    let max = str.lines().max_by_key(|line| line.len()).unwrap().len() + 1;

    let grid = str
        .lines()
        .map(|line| format!("{line:max$}"))
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let ops = &grid[grid.len() - 1];
    let grid = &grid[..grid.len() - 1];
    let mut ops = ops
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '+' || **c == '*')
        .collect_vec();

    let mut p2 = 0;

    for (i, &(x, op)) in ops.iter().enumerate() {
        let next = ops.get(i + 1).map(|(i, _)| *i).unwrap_or(grid[0].len());
        let len = next - x - 1;
        let mut result = vec![vec![' '; grid.len()]; len];

        for (y, row) in grid.iter().enumerate() {
            let row = &grid[y][x..next];
            let row = &row[..row.len() - 1];

            for (i, c) in row.iter().enumerate() {
                result[i][y] = *c;
            }
        }

        // println!("{result:?}");

        let result = result
            .into_iter()
            .map(|col| col.into_iter().filter(|c| *c != ' ').collect::<String>())
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec();

        // println!("{op} {result:?}");

        match *op {
            '+' => p2 += result.iter().sum::<u64>(),
            '*' => p2 += result.iter().product::<u64>(),
            _ => unreachable!(),
        }
    }

    part2!(results, "{p2}");

    Ok(())
}
