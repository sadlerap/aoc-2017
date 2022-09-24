use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
fn generator(input: &[u8]) -> Vec<u8> {
    input.iter()
        .map(|c| c - b'0')
        .collect()
        
}

fn solve(input: &[u8], offset: usize) -> i32 {
    let offset = offset % input.len();
    input.iter()
        .zip(input[offset..].iter().chain(input[0..offset].iter()))
        .filter_map(|(a, b)| if a == b {Some(i32::from(*a))} else {None})
        .sum()
}

#[aoc(day1, part1)]
fn part_1(input: &[u8]) -> i32 {
    solve(input, 1)
}

#[aoc(day1, part2)]
fn part_2(input: &[u8]) -> i32 {
    solve(input, input.len() / 2)
}
