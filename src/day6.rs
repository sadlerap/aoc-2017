//! --- Day 6: Memory Reallocation ---
//!
//! A debugger program here is having an issue: it is trying to repair a memory reallocation
//! routine, but it keeps getting stuck in an infinite loop.
//!
//! In this area, there are sixteen memory banks; each memory bank can hold any number of blocks.
//! The goal of the reallocation routine is to balance the blocks between the memory banks.
//!
//! The reallocation routine operates in cycles. In each cycle, it finds the memory bank with the
//! most blocks (ties won by the lowest-numbered memory bank) and redistributes those blocks among
//! the banks. To do this, it removes all of the blocks from the selected bank, then moves to the
//! next (by index) memory bank and inserts one of the blocks. It continues doing this until it
//! runs out of blocks; if it reaches the last memory bank, it wraps around to the first one.
//!
//! The debugger would like to know how many redistributions can be done before a blocks-in-banks
//! configuration is produced that has been seen before.
//!
//! For example, imagine a scenario with only four memory banks:
//!
//! - The banks start with `0`, `2`, `7`, and `0` blocks. The third bank has the most blocks, so it is
//! chosen for redistribution.
//! - Starting with the next bank (the fourth bank) and then continuing to the first bank, the
//! second bank, and so on, the 7 blocks are spread out over the memory banks. The fourth, first,
//! and second banks get two blocks each, and the third bank gets one back. The final result looks
//! like this: `2 4 1 2`.
//! - Next, the second bank is chosen because it contains the most blocks (four). Because there are
//! four memory banks, each gets one block. The result is: `3 1 2 3`.
//! - Now, there is a tie between the first and fourth memory banks, both of which have three
//! blocks. The first bank wins the tie, and its three blocks are distributed evenly over the other
//! three banks, leaving it with none: `0 2 3 4`.
//! - The fourth bank is chosen, and its four blocks are distributed such that each of the four
//! banks receives one: `1 3 4 1`.
//! - The third bank is chosen, and the same thing happens: `2 4 1 2`.
//!
//! At this point, we've reached a state we've seen before: `2 4 1 2` was already seen. The
//! infinite loop is detected after the fifth block redistribution cycle, and so the answer in this
//! example is 5.
//!
//! Given the initial block counts in your puzzle input, how many redistribution cycles must be
//! completed before a configuration is produced that has been seen before?

use anyhow::anyhow;
use aoc_runner_derive::aoc;
use fxhash::{FxHashMap, FxHashSet};
use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

fn parse(input: &str) -> anyhow::Result<Vec<u8>> {
    let res: IResult<_, _> = separated_list1(space1, map_res(digit1, str::parse::<u8>))(input);
    res.map(|x| x.1)
        .map_err(|_| anyhow!("Unexpected non-integer in input"))
}

#[inline]
fn step(state: &mut [u8]) {
    let len = state.len();
    let (i, &val) = state
        .iter()
        .enumerate()
        .max_by(|(i, x), (j, y)| {
            let res = x.cmp(y);
            if res.is_eq() {
                j.cmp(i)
            } else {
                res
            }
        })
        .unwrap();

    state[i] = 0;

    let cutoff = usize::from(val) % len;
    let to_write = u8::try_from(usize::from(val) / len).unwrap();
    std::iter::repeat(0..len)
        .flatten()
        .skip(i + 1)
        .take(len)
        .enumerate()
        .for_each(|(i, x)| {
            if i >= cutoff {
                state[x] += to_write;
            } else {
                state[x] += to_write + 1;
            }
        })
}

#[aoc(day6, part1)]
fn part1(input: &str) -> anyhow::Result<u32> {
    let mut data = parse(input)?;
    let mut seen = FxHashSet::default();
    for i in 0.. {
        let key = fxhash::hash32(&data);
        if !seen.insert(key) {
            return Ok(i);
        }
        step(&mut data);
    }
    unreachable!()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> anyhow::Result<u32> {
    let mut data = parse(input)?;
    let mut seen = FxHashMap::default();

    for i in 0_u32.. {
        let key = fxhash::hash32(&data);
        if let Some(initial_pass) = seen.get(&key) {
            return Ok(i - initial_pass);
        } else {
            seen.insert(key, i);
        }
        step(&mut data);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "0 2 7 0";
        assert_eq!(part1(input).unwrap(), 5);
    }

    #[test]
    fn example_part2() {
        let input = "0 2 7 0";
        assert_eq!(part2(input).unwrap(), 4);
    }

    #[test]
    fn part1_step() {
        let mut input = [0, 2, 7, 0];
        step(&mut input);
        assert_eq!(input, [2, 4, 1, 2]);
        step(&mut input);
        assert_eq!(input, [3, 1, 2, 3]);
        step(&mut input);
        assert_eq!(input, [0, 2, 3, 4]);
        step(&mut input);
        assert_eq!(input, [1, 3, 4, 1]);
        step(&mut input);
        assert_eq!(input, [2, 4, 1, 2]);
    }
}
