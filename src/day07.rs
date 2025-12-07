use crate::prelude::*;

pub fn run(input: &str, results: &mut OutputResults) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = grid[0].iter().position(|c| *c == 'S').unwrap();
    let mut beams = FxHashSet::default();
    beams.insert(start);

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
}

pub(crate) fn bench_part1(input: &str) {
    panic!("unimplemented");
}

pub(crate) fn bench_part2(input: &str) {
    let mut grid = input.lines();

    let start = grid.next().unwrap().len() / 2;

    let mut sum_buf = [0; 256];
    let mut beams = FxHashSet::with_capacity_and_hasher(256, FxBuildHasher);

    beams.insert(start);
    sum_buf[start] = 1;

    for row in grid.map(str::bytes) {
        for (x, c) in row.enumerate() {
            if c == b'^' && beams.contains(&x) {
                let curr = sum_buf[x];

                sum_buf[x] = 0;
                sum_buf[x + 1] += curr;
                sum_buf[x - 1] += curr;

                beams.remove(&x);
                beams.insert(x + 1);
                beams.insert(x - 1);
            }
        }
    }

    let sum = sum_buf.iter().sum::<usize>();

    // Day 7 Part 2 (5.0s) ...               35_833.189 ns/iter (0.999 R²)
    // Day 7 Part 2 (5.0s) ...               26_296.319 ns/iter (0.997 R²)
    // Day 7 Part 2 (5.0s) ...               20_137.905 ns/iter (0.998 R²)
    // Day 7 Part 2 (5.0s) ...               18_698.586 ns/iter (1.000 R²)
    assert_eq!(sum, 3223365367809);
}
