use crate::prelude::*;

pub fn run(input: &str, results: &mut OutputResults) {
    let input = input.lines().map(|line| {
        let (dir, n) = line.split_at(1);
        let dir = match dir {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };

        n.parse::<i64>().unwrap() * dir
    });

    let p1 = input
        .clone()
        .scan(50i64, |state, n| {
            *state = (*state + n).rem_euclid(100);
            Some(*state)
        })
        .filter(|n| *n == 0)
        .count() as i64;

    let mut acc = 50;
    let mut p2 = 0;

    for n in input {
        acc += n;

        if acc <= 0 && n != acc {
            p2 += 1;
        }

        p2 += acc.abs() / 100;
        acc = acc.rem_euclid(100);
    }

    part1!(results, "{p1}");
    part2!(results, "{p2}");
}

pub(crate) fn bench_part1(input: &str) {
    panic!("unimplemented");
}

pub(crate) fn bench_part2(input: &str) {
    panic!("unimplemented");
}