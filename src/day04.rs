use crate::prelude::*;

type Grid = Vec<Vec<char>>;

fn get_xy(grid: &Grid, x: isize, y: isize) -> Option<&char> {
    if x.is_negative() || y.is_negative() {
        return None;
    }

    grid.get(y as usize).and_then(|row| row.get(x as usize))
}

#[rustfmt::skip]
fn get_adjacent_and_self(grid: &Grid, x: usize, y: usize) -> [Option<&char>; 9] {
    let (x, y) = (x as isize, y as isize);

    [
        get_xy(grid, x       , y     ),
        get_xy(grid, x + 1, y      ),
        get_xy(grid, x - 1, y      ),
        get_xy(grid, x      , y + 1),
        get_xy(grid, x      , y - 1),
        get_xy(grid, x + 1, y + 1),
        get_xy(grid, x - 1, y - 1),
        get_xy(grid, x + 1, y - 1),
        get_xy(grid, x - 1, y + 1),
    ]
}

pub fn run(input: &str, results: &mut OutputResults) -> anyhow::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut p1 = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            let adjacent = get_adjacent_and_self(&grid, x, y)
                .into_iter()
                .flatten()
                .filter(|c| **c == '@')
                .count();

            if adjacent <= 4 {
                p1 += 1;
            }
        }
    }

    part1!(results, "{p1}");

    let mut p2 = 0;

    let mut grid_current = grid;

    loop {
        let mut check = false;
        let grid = grid_current.clone();

        for (y, row) in grid_current.iter_mut().enumerate() {
            for (x, c) in row.iter_mut().enumerate() {
                if *c == '.' {
                    continue;
                }

                let adjacent = get_adjacent_and_self(&grid, x, y)
                    .into_iter()
                    .flatten()
                    .filter(|c| **c == '@')
                    .count();

                if adjacent <= 4 {
                    p2 += 1;
                    *c = '.';
                    check = true;
                }
            }
        }

        if !check {
            break;
        }
    }

    part2!(results, "{p2}");

    Ok(())
}
